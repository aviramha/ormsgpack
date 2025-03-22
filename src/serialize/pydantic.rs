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

pub enum PydanticModelError {
    DictMissing,
}

impl std::fmt::Display for PydanticModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DictMissing => write!(f, "Pydantic's BaseModel must have __dict__ attribute"),
        }
    }
}

pub struct PydanticModel {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl PydanticModel {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Result<Self, PydanticModelError> {
        let dict = ffi!(PyObject_GetAttr(ptr, DICT_STR));
        if unlikely!(dict.is_null()) {
            ffi!(PyErr_Clear());
            return Err(PydanticModelError::DictMissing);
        }
        Ok(PydanticModel {
            ptr: dict,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        })
    }
}

impl Drop for PydanticModel {
    fn drop(&mut self) {
        ffi!(Py_DECREF(self.ptr));
    }
}

impl Serialize for PydanticModel {
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
