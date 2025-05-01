// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::int::*;
use pyo3::ffi::*;

impl Int {
    pub fn new(op: *mut PyObject) -> Result<Self, IntError> {
        match pylong_to_i64(op) {
            Some(val) => Ok(Int::Signed(val)),
            None => match pylong_to_u64(op) {
                Some(val) => Ok(Int::Unsigned(val)),
                None => Err(IntError::Overflow),
            },
        }
    }
}
