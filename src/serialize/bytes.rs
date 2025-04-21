// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::pybytes_as_bytes;
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
        let contents = unsafe { pybytes_as_bytes(self.ptr) };
        serializer.serialize_bytes(contents)
    }
}
