// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::pybytearray_as_bytes;
use serde::ser::{Serialize, Serializer};

#[repr(transparent)]
pub struct ByteArray {
    ptr: *mut pyo3::ffi::PyObject,
}

impl ByteArray {
    pub fn new(ptr: *mut pyo3::ffi::PyObject) -> Self {
        ByteArray { ptr }
    }
}

impl Serialize for ByteArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let contents = unsafe { pybytearray_as_bytes(self.ptr) };
        serializer.serialize_bytes(contents)
    }
}
