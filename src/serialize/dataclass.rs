// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::exc::*;
use crate::ffi::PyDictIter;
use crate::opt::*;
use crate::serialize::serializer::*;
use crate::typeref::*;
use crate::unicode::*;

use serde::ser::{Serialize, SerializeMap, Serializer};

use std::ptr::NonNull;

pub struct DataclassGenericSerializer {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl DataclassGenericSerializer {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        DataclassGenericSerializer {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for DataclassGenericSerializer {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dict = ffi!(PyObject_GetAttr(self.ptr, DICT_STR));
        let ob_type = ob_type!(self.ptr);
        if unlikely!(dict.is_null()) {
            ffi!(PyErr_Clear());
            DataclassFallbackSerializer::new(
                self.ptr,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer)
        } else if pydict_contains!(ob_type, SLOTS_STR) {
            let ret = DataclassFallbackSerializer::new(
                self.ptr,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer);
            ffi!(Py_DECREF(dict));
            ret
        } else {
            let ret = DataclassFastSerializer::new(
                dict,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer);
            ffi!(Py_DECREF(dict));
            ret
        }
    }
}

pub struct DataclassFastSerializer {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl DataclassFastSerializer {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        DataclassFastSerializer {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for DataclassFastSerializer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = ffi!(Py_SIZE(self.ptr));
        if unlikely!(len == 0) {
            return serializer.serialize_map(Some(0)).unwrap().end();
        }
        let mut map = serializer.serialize_map(None).unwrap();
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

            let pyvalue = PyObjectSerializer::new(
                value.as_ptr(),
                self.opts,
                self.default_calls,
                self.recursion + 1,
                self.default,
            );
            map.serialize_key(key_as_str).unwrap();
            map.serialize_value(&pyvalue)?;
        }
        map.end()
    }
}

pub struct DataclassFallbackSerializer {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl DataclassFallbackSerializer {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        DataclassFallbackSerializer {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for DataclassFallbackSerializer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fields = ffi!(PyObject_GetAttr(self.ptr, DATACLASS_FIELDS_STR));
        ffi!(Py_DECREF(fields));
        let len = ffi!(Py_SIZE(fields));
        if unlikely!(len == 0) {
            return serializer.serialize_map(Some(0)).unwrap().end();
        }
        let mut map = serializer.serialize_map(None).unwrap();
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
            let pyvalue = PyObjectSerializer::new(
                value,
                self.opts,
                self.default_calls,
                self.recursion + 1,
                self.default,
            );
            map.serialize_key(key_as_str).unwrap();
            map.serialize_value(&pyvalue)?
        }
        map.end()
    }
}
