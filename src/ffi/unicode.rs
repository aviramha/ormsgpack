// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::ffi::*;

#[inline(never)]
pub fn unicode_to_str_via_ffi(op: *mut PyObject) -> Option<&'static str> {
    unsafe {
        let mut size: Py_ssize_t = 0;
        let ptr = PyUnicode_AsUTF8AndSize(op, &mut size).cast::<u8>();
        if unlikely!(ptr.is_null()) {
            PyErr_Clear();
            None
        } else {
            let slice = std::slice::from_raw_parts(ptr, size as usize);
            Some(std::str::from_utf8_unchecked(slice))
        }
    }
}
