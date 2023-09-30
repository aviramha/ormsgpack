// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::exc::*;
use crate::ffi::PyDict_GET_SIZE;
use crate::opt::*;
use crate::serialize::serializer::pyobject_to_obtype;
use crate::serialize::serializer::*;
use crate::typeref::*;
use crate::unicode::*;
use serde::ser::{Serialize, SerializeMap, Serializer};
use std::ptr::NonNull;

pub struct Dict {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl Dict {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        Dict {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for Dict {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = unsafe { PyDict_GET_SIZE(self.ptr) as usize };
        let mut map = serializer.serialize_map(Some(len)).unwrap();
        let mut pos = 0isize;
        let mut key: *mut pyo3::ffi::PyObject = std::ptr::null_mut();
        let mut value: *mut pyo3::ffi::PyObject = std::ptr::null_mut();
        for _ in 0..=len - 1 {
            unsafe {
                pyo3::ffi::_PyDict_Next(
                    self.ptr,
                    &mut pos,
                    &mut key,
                    &mut value,
                    std::ptr::null_mut(),
                )
            };
            let value = PyObjectSerializer::new(
                value,
                self.opts,
                self.default_calls,
                self.recursion + 1,
                self.default,
            );
            if unlikely!(!is_type!(ob_type!(key), STR_TYPE)) {
                err!(KEY_MUST_BE_STR)
            }
            {
                let data = unicode_to_str(key);
                if unlikely!(data.is_none()) {
                    err!(INVALID_STR)
                }
                map.serialize_key(data.unwrap()).unwrap();
            }

            map.serialize_value(&value)?;
        }
        map.end()
    }
}

pub struct DictNonStrKey {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl DictNonStrKey {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        DictNonStrKey {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for DictNonStrKey {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = unsafe { PyDict_GET_SIZE(self.ptr) as usize };
        let mut pos = 0isize;
        let mut key: *mut pyo3::ffi::PyObject = std::ptr::null_mut();
        let mut value: *mut pyo3::ffi::PyObject = std::ptr::null_mut();
        let opts = self.opts & NOT_PASSTHROUGH;
        let mut map = serializer.serialize_map(None).unwrap();
        for _ in 0..=len - 1 {
            unsafe {
                pyo3::ffi::_PyDict_Next(
                    self.ptr,
                    &mut pos,
                    &mut key,
                    &mut value,
                    std::ptr::null_mut(),
                )
            };
            if is_type!(ob_type!(key), STR_TYPE) {
                let data = unicode_to_str(key);
                if unlikely!(data.is_none()) {
                    err!(INVALID_STR)
                }
                map.serialize_entry(
                    data.unwrap(),
                    &PyObjectSerializer::new(
                        value,
                        self.opts,
                        self.default_calls,
                        self.recursion + 1,
                        self.default,
                    ),
                )?;
            } else {
                match pyobject_to_obtype(key, opts) {
                    ObType::NumpyScalar
                    | ObType::NumpyArray
                    | ObType::Dict
                    | ObType::List
                    | ObType::Dataclass
                    | ObType::Pydantic
                    | ObType::Unknown => {
                        err!("Dict key must a type serializable with OPT_NON_STR_KEYS")
                    }
                    _ => (),
                }
                map.serialize_entry(
                    &PyObjectSerializer::new(
                        key,
                        opts,
                        self.default_calls,
                        self.recursion + 1,
                        self.default,
                    ),
                    &PyObjectSerializer::new(
                        value,
                        self.opts,
                        self.default_calls,
                        self.recursion + 1,
                        self.default,
                    ),
                )?;
            }
        }
        map.end()
    }
}
