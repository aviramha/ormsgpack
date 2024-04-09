// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::opt::*;
use crate::serialize::serializer::*;

use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::ptr::NonNull;

pub struct ListSerializer {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl ListSerializer {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        ListSerializer {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for ListSerializer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = ffi!(PyList_GET_SIZE(self.ptr)) as usize;
        let mut seq = serializer.serialize_seq(Some(len)).unwrap();
        for i in 0..len {
            let item = ffi!(PyList_GET_ITEM(self.ptr, i as isize));
            let value = PyObjectSerializer::new(
                item,
                self.opts,
                self.default_calls,
                self.recursion + 1,
                self.default,
            );
            seq.serialize_element(&value)?;
        }
        seq.end()
    }
}
