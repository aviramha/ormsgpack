// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::exc::*;
use crate::unicode::*;

use serde::ser::{Serialize, Serializer};

#[repr(transparent)]
pub struct Str {
    ptr: *mut pyo3::ffi::PyObject,
}

impl Str {
    pub fn new(ptr: *mut pyo3::ffi::PyObject) -> Self {
        Str { ptr: ptr }
    }
}

impl Serialize for Str {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let uni = unicode_to_str(self.ptr);
        if unlikely!(uni.is_none()) {
            err!(INVALID_STR)
        }
        serializer.serialize_str(uni.unwrap())
    }
}

#[repr(transparent)]
pub struct StrSubclass {
    ptr: *mut pyo3::ffi::PyObject,
}

impl StrSubclass {
    pub fn new(ptr: *mut pyo3::ffi::PyObject) -> Self {
        StrSubclass { ptr: ptr }
    }
}

impl Serialize for StrSubclass {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let uni = unicode_to_str_via_ffi(self.ptr);
        if unlikely!(uni.is_none()) {
            err!(INVALID_STR)
        }
        serializer.serialize_str(uni.unwrap())
    }
}
