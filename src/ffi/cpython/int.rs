// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::int::{Int, IntError};
use pyo3::ffi::*;

#[repr(C)]
#[cfg(Py_3_12)]
struct _PyLongValue {
    pub lv_tag: usize,
}

#[repr(C)]
#[cfg(Py_3_12)]
struct PyLongObject {
    pub ob_base: PyObject,
    pub long_value: _PyLongValue,
}

#[cfg(Py_3_12)]
const SIGN_MASK: usize = 3;

#[cfg(Py_3_12)]
pub fn pylong_is_positive(op: *mut PyObject) -> bool {
    unsafe { (*(op as *mut PyLongObject)).long_value.lv_tag & SIGN_MASK == 0 }
}

#[cfg(not(Py_3_12))]
pub fn pylong_is_positive(op: *mut PyObject) -> bool {
    unsafe { (*(op as *mut PyVarObject)).ob_size > 0 }
}

impl Int {
    pub fn new(op: *mut pyo3::ffi::PyObject) -> Result<Self, IntError> {
        unsafe {
            if pylong_is_positive(op) {
                let val = PyLong_AsUnsignedLongLong(op);
                if unlikely!(val == u64::MAX && !PyErr_Occurred().is_null()) {
                    PyErr_Clear();
                    Err(IntError::Overflow)
                } else {
                    Ok(Int::Unsigned(val))
                }
            } else {
                let val = PyLong_AsLongLong(op);
                if unlikely!(val == -1 && !PyErr_Occurred().is_null()) {
                    PyErr_Clear();
                    Err(IntError::Overflow)
                } else {
                    Ok(Int::Signed(val))
                }
            }
        }
    }
}
