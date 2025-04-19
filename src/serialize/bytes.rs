// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::*;

use serde::ser::{Serialize, Serializer};

#[repr(transparent)]
pub struct Bytes {
    ptr: *mut pyo3::ffi::PyObject,
}

impl Bytes {
    pub fn new(ptr: *mut pyo3::ffi::PyObject) -> Self {
        Bytes { ptr: ptr }
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let buffer = unsafe { pybytes_as_u8(self.ptr) };
        let length = unsafe { pyo3::ffi::Py_SIZE(self.ptr) as usize };
        let contents = unsafe { std::slice::from_raw_parts(buffer, length) };
        serializer.serialize_bytes(contents)
    }
}
