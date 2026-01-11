// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::ffi::*;
use std::ffi::c_void;

// The unicode object state
//
// https://github.com/python/cpython/blob/v3.14.2/Include/cpython/unicodeobject.h#L107
//
// has an implementation defined layout because it contains
// bit-fields. The functions in this module support the following
// layouts:
//
// GIL enabled
// x86_64, arm64
// GCC, Clang, MSVC
//
// | padding | statically_allocated | ascii | compact | kind | interned |
// |---------+----------------------+-------+---------+------+----------|
// |      24 |                    1 |     1 |       1 |    3 |        2 |
//
// GIL disabled
// x86_64, arm64
// GCC, Clang
//
// | padding | statically_allocated | ascii | compact | kind | interned |
// |---------+----------------------+-------+---------+------+----------|
// |      18 |                    1 |     1 |       1 |    3 |        8 |

#[cfg(all(Py_3_14, Py_GIL_DISABLED))]
const STATE_KIND_INDEX: usize = 8;

#[cfg(not(all(Py_3_14, Py_GIL_DISABLED)))]
const STATE_KIND_INDEX: usize = 2;

const STATE_COMPACT_INDEX: usize = STATE_KIND_INDEX + 3;

const STATE_ASCII_INDEX: usize = STATE_COMPACT_INDEX + 1;

#[inline(always)]
pub unsafe fn pyunicode_kind(op: *mut PyObject) -> u32 {
    let state = (*op.cast::<PyASCIIObject>()).state;
    (state >> STATE_KIND_INDEX) & 7
}

#[inline(always)]
pub unsafe fn pyunicode_is_compact(op: *mut PyObject) -> bool {
    let state = (*op.cast::<PyASCIIObject>()).state;
    state & (1 << STATE_COMPACT_INDEX) != 0
}

#[inline(always)]
pub unsafe fn pyunicode_is_ascii(op: *mut PyObject) -> bool {
    let state = (*op.cast::<PyASCIIObject>()).state;
    state & (1 << STATE_ASCII_INDEX) != 0
}

#[inline(always)]
pub unsafe fn pyunicode_compact_data(op: *mut PyObject) -> *mut c_void {
    debug_assert!(pyunicode_is_compact(op));
    if pyunicode_is_ascii(op) {
        op.cast::<PyASCIIObject>().offset(1).cast::<c_void>()
    } else {
        op.cast::<PyCompactUnicodeObject>()
            .offset(1)
            .cast::<c_void>()
    }
}
