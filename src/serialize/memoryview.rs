// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::pymemoryview_as_bytes;
use serde::ser::{Serialize, Serializer};

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
        if let Some(contents) = unsafe { pymemoryview_as_bytes(self.ptr) } {
            serializer.serialize_bytes(contents)
        } else {
            Err(serde::ser::Error::custom(
                "Failed to get buffer from memoryview",
            ))
        }
    }
}
