// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::unicode::*;
use pyo3::ffi::*;

#[inline(always)]
pub fn hash_str(op: *mut PyObject) -> Py_hash_t {
    unsafe { PyObject_Hash(op) }
}

#[inline(always)]
pub fn unicode_from_str(buf: &str) -> *mut PyObject {
    unsafe { PyUnicode_FromStringAndSize(buf.as_ptr().cast::<i8>(), buf.len() as isize) }
}

#[inline(always)]
pub fn unicode_to_str(op: *mut PyObject) -> Result<&'static str, UnicodeError> {
    unicode_to_str_via_ffi(op)
}
