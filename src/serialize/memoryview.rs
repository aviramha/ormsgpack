// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use serde::ser::{Serialize, Serializer};

use crate::ffi::PyMemoryView_GET_BUFFER;

#[repr(transparent)]
pub struct MemoryView {
    ptr: *mut pyo3::ffi::PyObject,
}

impl MemoryView {
    pub fn new(ptr: *mut pyo3::ffi::PyObject) -> Self {
        MemoryView { ptr }
    }
}

impl Serialize for MemoryView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let buffer = unsafe { PyMemoryView_GET_BUFFER(self.ptr) };
        if buffer.is_null() {
            return Err(serde::ser::Error::custom(
                "Failed to get buffer from memoryview",
            ));
        }
        let length = unsafe { (*buffer).len };
        let contents =
            unsafe { std::slice::from_raw_parts((*buffer).buf as *const u8, length as usize) };

        serializer.serialize_bytes(contents)
    }
}
