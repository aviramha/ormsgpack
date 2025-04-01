// SPDX-License-Identifier: (Apache-2.0 OR MIT)

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
        let buffer = unsafe { pyo3::ffi::PyByteArray_AsString(self.ptr) } as *const u8;
        let length = unsafe { pyo3::ffi::PyByteArray_Size(self.ptr) };
        let contents = unsafe { std::slice::from_raw_parts(buffer, length as usize) };
        serializer.serialize_bytes(contents)
    }
}
