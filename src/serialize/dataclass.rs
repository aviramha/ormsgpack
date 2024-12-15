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
            if unlikely!(!py_is!(ob_type!(key.as_ptr()), STR_TYPE)) {
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

        if self.opts & SORT_KEYS != 0 {
            items.sort_unstable_by(|a, b| a.0.cmp(b.0));
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

fn is_pseudo_field(field: *mut pyo3::ffi::PyObject) -> bool {
    let field_type = ffi!(PyObject_GetAttr(field, FIELD_TYPE_STR));
    ffi!(Py_DECREF(field_type));
    !py_is!(field_type as *mut pyo3::ffi::PyTypeObject, FIELD_TYPE)
}

impl Serialize for Dataclass {
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

        let dict = {
            let ob_type = ob_type!(self.ptr);
            if pydict_contains!(ob_type, SLOTS_STR) {
                std::ptr::null_mut()
            } else {
                let dict = ffi!(PyObject_GetAttr(self.ptr, DICT_STR));
                ffi!(Py_DECREF(dict));
                dict
            }
        };

        let mut items: SmallVec<[(&str, *mut pyo3::ffi::PyObject); 8]> =
            SmallVec::with_capacity(len);
        for (attr, field) in PyDictIter::from_pyobject(fields) {
            let data = unicode_to_str(attr.as_ptr());
            if unlikely!(data.is_none()) {
                err!(INVALID_STR);
            }
            let key_as_str = data.unwrap();
            if key_as_str.as_bytes()[0] == b'_' {
                continue;
            }

            if unlikely!(dict.is_null()) {
                if !is_pseudo_field(field.as_ptr()) {
                    let value = ffi!(PyObject_GetAttr(self.ptr, attr.as_ptr()));
                    ffi!(Py_DECREF(value));
                    items.push((key_as_str, value));
                }
            } else {
                let value = ffi!(PyDict_GetItem(dict, attr.as_ptr()));
                if !value.is_null() {
                    items.push((key_as_str, value));
                } else if !is_pseudo_field(field.as_ptr()) {
                    let value = ffi!(PyObject_GetAttr(self.ptr, attr.as_ptr()));
                    ffi!(Py_DECREF(value));
                    items.push((key_as_str, value));
                }
            }
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
