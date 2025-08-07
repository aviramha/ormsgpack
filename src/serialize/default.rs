// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::*;
use crate::opt::*;
use crate::serialize::serializer::*;
use crate::state::State;

use serde::ser::{Serialize, Serializer};
use std::ffi::CStr;

use std::ptr::NonNull;

#[cold]
#[inline(never)]
fn format_err(ptr: *mut pyo3::ffi::PyObject) -> String {
    let name = unsafe { CStr::from_ptr((*ob_type!(ptr)).tp_name).to_string_lossy() };
    format_args!("Type is not msgpack serializable: {name}").to_string()
}

pub struct Default {
    ptr: *mut pyo3::ffi::PyObject,
    state: *mut State,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl Default {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        state: *mut State,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        Default {
            ptr: ptr,
            state: state,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for Default {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.default {
            Some(callable) => {
                if unlikely!(self.default_calls == RECURSION_LIMIT) {
                    return Err(serde::ser::Error::custom(
                        "default serializer exceeds recursion limit",
                    ));
                }
                let default_obj = unsafe { pyobject_call_one_arg(callable.as_ptr(), self.ptr) };
                if unlikely!(default_obj.is_null()) {
                    Err(serde::ser::Error::custom(format_err(self.ptr)))
                } else {
                    let res = PyObject::new(
                        default_obj,
                        self.state,
                        self.opts,
                        self.default_calls + 1,
                        self.recursion,
                        self.default,
                    )
                    .serialize(serializer);
                    unsafe { pyo3::ffi::Py_DECREF(default_obj) };
                    res
                }
            }
            None => Err(serde::ser::Error::custom(format_err(self.ptr))),
        }
    }
}
