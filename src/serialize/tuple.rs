// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::*;
use crate::opt::*;
use crate::serialize::serializer::*;
use crate::state::State;

use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::ptr::NonNull;

pub struct Tuple {
    ptr: *mut pyo3::ffi::PyObject,
    state: *mut State,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl Tuple {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        state: *mut State,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        Tuple {
            ptr: ptr,
            state: state,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for Tuple {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = unsafe { pyo3::ffi::Py_SIZE(self.ptr) } as usize;
        let mut seq = serializer.serialize_seq(Some(len)).unwrap();
        for i in 0..len {
            let item = unsafe { pytuple_get_item(self.ptr, i as isize) };
            let value = PyObject::new(
                item,
                self.state,
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
