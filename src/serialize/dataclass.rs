// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::exc::*;
use crate::ffi::PyDictIter;
use crate::opt::*;
use crate::serialize::serializer::*;
use crate::typeref::*;
use crate::unicode::*;

use serde::ser::{Serialize, SerializeMap, Serializer};

use smallvec::SmallVec;
use std::ptr::NonNull;

pub struct Dataclass {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl Dataclass {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        Dataclass {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for Dataclass {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ob_type = ob_type!(self.ptr);
        if pydict_contains!(ob_type, SLOTS_STR) {
            DataclassFields::new(
                self.ptr,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer)
        } else {
            match AttributeDict::new(
                self.ptr,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            ) {
                Ok(val) => val.serialize(serializer),
                Err(AttributeDictError::DictMissing) => DataclassFields::new(
                    self.ptr,
                    self.opts,
                    self.default_calls,
                    self.recursion,
                    self.default,
                )
                .serialize(serializer),
            }
        }
    }
}

pub enum AttributeDictError {
    DictMissing,
}

pub struct AttributeDict {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl AttributeDict {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Result<Self, AttributeDictError> {
        let dict = ffi!(PyObject_GetAttr(ptr, DICT_STR));
        if unlikely!(dict.is_null()) {
            ffi!(PyErr_Clear());
            return Err(AttributeDictError::DictMissing);
        }
        Ok(AttributeDict {
            ptr: dict,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        })
    }
}

impl Drop for AttributeDict {
    fn drop(&mut self) {
        ffi!(Py_DECREF(self.ptr));
    }
}

impl Serialize for AttributeDict {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = ffi!(Py_SIZE(self.ptr)) as usize;
        if unlikely!(len == 0) {
            return serializer.serialize_map(Some(0)).unwrap().end();
        }
        let mut items: SmallVec<[(&str, *mut pyo3::ffi::PyObject); 8]> =
            SmallVec::with_capacity(len);
        for (key, value) in PyDictIter::from_pyobject(self.ptr) {
            if unlikely!(!is_type!(ob_type!(key.as_ptr()), STR_TYPE)) {
                err!(KEY_MUST_BE_STR)
            }
            let data = unicode_to_str(key.as_ptr());
            if unlikely!(data.is_none()) {
                err!(INVALID_STR)
            }
            let key_as_str = data.unwrap();
            if unlikely!(key_as_str.as_bytes()[0] == b'_') {
                continue;
            }
            items.push((key_as_str, value.as_ptr()));
        }

        let mut map = serializer.serialize_map(Some(items.len())).unwrap();
        for (key, value) in items.iter() {
            let pyvalue = PyObject::new(
                *value,
                self.opts,
                self.default_calls,
                self.recursion + 1,
                self.default,
            );
            map.serialize_key(key).unwrap();
            map.serialize_value(&pyvalue)?;
        }
        map.end()
    }
}

pub struct DataclassFields {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl DataclassFields {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        DataclassFields {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for DataclassFields {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fields = ffi!(PyObject_GetAttr(self.ptr, DATACLASS_FIELDS_STR));
        ffi!(Py_DECREF(fields));
        let len = ffi!(Py_SIZE(fields)) as usize;
        if unlikely!(len == 0) {
            return serializer.serialize_map(Some(0)).unwrap().end();
        }
        let mut items: SmallVec<[(&str, *mut pyo3::ffi::PyObject); 8]> =
            SmallVec::with_capacity(len);
        for (attr, field) in PyDictIter::from_pyobject(fields) {
            let field_type = ffi!(PyObject_GetAttr(field.as_ptr(), FIELD_TYPE_STR));
            ffi!(Py_DECREF(field_type));
            if !is_type!(field_type as *mut pyo3::ffi::PyTypeObject, FIELD_TYPE) {
                continue;
            }
            let data = unicode_to_str(attr.as_ptr());
            if unlikely!(data.is_none()) {
                err!(INVALID_STR);
            }
            let key_as_str = data.unwrap();
            if key_as_str.as_bytes()[0] == b'_' {
                continue;
            }

            let value = ffi!(PyObject_GetAttr(self.ptr, attr.as_ptr()));
            ffi!(Py_DECREF(value));
            items.push((key_as_str, value));
        }

        let mut map = serializer.serialize_map(Some(items.len())).unwrap();
        for (key, value) in items.iter() {
            let pyvalue = PyObject::new(
                *value,
                self.opts,
                self.default_calls,
                self.recursion + 1,
                self.default,
            );
            map.serialize_key(key).unwrap();
            map.serialize_value(&pyvalue)?
        }
        map.end()
    }
}
