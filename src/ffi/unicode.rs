// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::ffi::*;

#[inline(never)]
pub fn unicode_to_str_via_ffi(op: *mut PyObject) -> Option<&'static str> {
    let mut size: Py_ssize_t = 0;
    let ptr = unsafe { PyUnicode_AsUTF8AndSize(op, &mut size) } as *const u8;
    if unlikely!(ptr.is_null()) {
        None
    } else {
        Some(str_from_slice!(ptr, size as usize))
    }
}
