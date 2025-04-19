// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::int::{Int, IntError};
use pyo3::ffi::*;

impl Int {
    pub fn new(op: *mut pyo3::ffi::PyObject) -> Result<Self, IntError> {
        unsafe {
            let val = PyLong_AsLongLong(op);
            if unlikely!(val == -1 && !PyErr_Occurred().is_null()) {
                PyErr_Clear();
                let val = PyLong_AsUnsignedLongLong(op);
                if unlikely!(val == u64::MAX && !PyErr_Occurred().is_null()) {
                    PyErr_Clear();
                    Err(IntError::Overflow)
                } else {
                    Ok(Int::Unsigned(val))
                }
            } else {
                Ok(Int::Signed(val))
            }
        }
    }
}
