// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::int::*;
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
pub unsafe fn pylong_is_positive(op: *mut PyObject) -> bool {
    (*op.cast::<PyLongObject>()).long_value.lv_tag & SIGN_MASK == 0
}

#[cfg(not(Py_3_12))]
pub unsafe fn pylong_is_positive(op: *mut PyObject) -> bool {
    (*op.cast::<PyVarObject>()).ob_size > 0
}

impl Int {
    pub fn new(op: *mut PyObject) -> Result<Self, IntError> {
        if unsafe { pylong_is_positive(op) } {
            match pylong_to_u64(op) {
                Some(val) => Ok(Int::Unsigned(val)),
                None => Err(IntError::Overflow),
            }
        } else {
            match pylong_to_i64(op) {
                Some(val) => Ok(Int::Signed(val)),
                None => Err(IntError::Overflow),
            }
        }
    }
}
