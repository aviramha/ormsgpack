// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::exc::*;
use crate::ffi::*;
use crate::opt::*;
use crate::serialize::serializer::*;
use crate::state::State;

use serde::ser::{Serialize, SerializeMap, Serializer};

use smallvec::SmallVec;
use std::ptr::NonNull;

#[inline]
pub fn is_pydantic_model(ob_type: *mut pyo3::ffi::PyTypeObject, state: *mut State) -> bool {
    unsafe {
        let tp_dict = (*ob_type).tp_dict;
        !tp_dict.is_null()
            && (pyo3::ffi::PyDict_Contains(tp_dict, (*state).fields_str) == 1
                || pyo3::ffi::PyDict_Contains(tp_dict, (*state).pydantic_validator_str) == 1)
    }
}

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
    state: *mut State,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl PydanticModel {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        state: *mut State,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Result<Self, PydanticModelError> {
        let dict = unsafe { pyo3::ffi::PyObject_GetAttr(ptr, (*state).dict_str) };
        if unlikely!(dict.is_null()) {
            unsafe { pyo3::ffi::PyErr_Clear() };
            return Err(PydanticModelError::DictMissing);
        }
        Ok(PydanticModel {
            ptr: dict,
            state: state,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        })
    }
}

impl Drop for PydanticModel {
    fn drop(&mut self) {
        unsafe { pyo3::ffi::Py_DECREF(self.ptr) };
    }
}

impl Serialize for PydanticModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = unsafe { pydict_size(self.ptr) } as usize;
        if unlikely!(len == 0) {
            return serializer.serialize_map(Some(0)).unwrap().end();
        }
        let mut items: SmallVec<[(&str, *mut pyo3::ffi::PyObject); 8]> =
            SmallVec::with_capacity(len);
        for (key, value) in PyDictIter::from_pyobject(self.ptr) {
            if unlikely!(ob_type!(key.as_ptr()) != &raw mut pyo3::ffi::PyUnicode_Type) {
                return Err(serde::ser::Error::custom(KEY_MUST_BE_STR));
            }
            let key_as_str = unicode_to_str(key.as_ptr()).map_err(serde::ser::Error::custom)?;
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
                self.state,
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
