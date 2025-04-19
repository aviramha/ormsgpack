// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::ffi::*;

#[inline(always)]
pub fn hash_str(op: *mut PyObject) -> Py_hash_t {
    unsafe { PyObject_Hash(op) }
}

#[inline(always)]
pub fn unicode_from_str(buf: &str) -> *mut PyObject {
    unsafe { PyUnicode_FromStringAndSize(buf.as_ptr() as *const i8, buf.len() as isize) }
}

#[inline(never)]
pub fn unicode_to_str_via_ffi(op: *mut PyObject) -> Option<&'static str> {
    let mut str_size: Py_ssize_t = 0;
    let ptr = unsafe { PyUnicode_AsUTF8AndSize(op, &mut str_size) } as *const u8;
    if unlikely!(ptr.is_null()) {
        None
    } else {
        Some(str_from_slice!(ptr, str_size as usize))
    }
}

#[inline(always)]
pub fn unicode_to_str(op: *mut PyObject) -> Option<&'static str> {
    unicode_to_str_via_ffi(op)
}
