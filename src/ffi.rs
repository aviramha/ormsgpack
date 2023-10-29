// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::ffi::*;
use std::os::raw::{c_char, c_int};
use std::ptr::NonNull;

#[allow(non_snake_case)]
#[inline(always)]
pub unsafe fn PyBytes_AS_STRING(op: *mut PyObject) -> *const c_char {
    &(*op.cast::<PyBytesObject>()).ob_sval as *const c_char
}

#[allow(non_snake_case)]
#[inline(always)]
pub unsafe fn PyBytes_GET_SIZE(op: *mut PyObject) -> Py_ssize_t {
    (*op.cast::<PyVarObject>()).ob_size
}

#[repr(C)]
pub struct _PyManagedBufferObject {
    pub ob_base: *mut PyObject,
    pub flags: c_int,
    pub exports: Py_ssize_t,
    pub master: *mut Py_buffer,
}

#[repr(C)]
pub struct PyMemoryViewObject {
    pub ob_base: PyVarObject,
    pub mbuf: *mut _PyManagedBufferObject,
    pub hash: Py_hash_t,
    pub flags: c_int,
    pub exports: Py_ssize_t,
    pub view: Py_buffer,
    pub weakreflist: *mut PyObject,
    pub ob_array: [Py_ssize_t; 1],
}

#[allow(non_snake_case)]
#[inline(always)]
pub unsafe fn PyMemoryView_GET_BUFFER(op: *mut PyObject) -> *const Py_buffer {
    &(*op.cast::<PyMemoryViewObject>()).view
}

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

pub struct PyDictIter {
    op: *mut PyObject,
    pos: isize,
}

impl PyDictIter {
    #[inline]
    pub fn from_pyobject(op: *mut PyObject) -> Self {
        PyDictIter { op: op, pos: 0 }
    }
}

impl Iterator for PyDictIter {
    type Item = (NonNull<PyObject>, NonNull<PyObject>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut key: *mut PyObject = std::ptr::null_mut();
        let mut value: *mut PyObject = std::ptr::null_mut();
        if unsafe { PyDict_Next(self.op, &mut self.pos, &mut key, &mut value) } == 1 {
            Some((nonnull!(key), nonnull!(value)))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = ffi!(Py_SIZE(self.op)) as usize;
        (len, Some(len))
    }
}
