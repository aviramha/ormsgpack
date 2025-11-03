// SPDX-License-Identifier: (Apache-2.0 OR MIT)

mod critical_section;
#[cfg_attr(any(PyPy, GraalPy), path = "base/mod.rs")]
#[cfg_attr(not(any(PyPy, GraalPy)), path = "cpython/mod.rs")]
mod impl_;
mod int;
mod unicode;

pub use critical_section::*;
pub use impl_::*;
pub use int::*;
pub use unicode::*;

use pyo3::ffi::*;
use std::os::raw::{c_char, c_int};
use std::ptr::NonNull;

#[inline(always)]
pub unsafe fn pybytes_as_bytes(op: *mut PyObject) -> &'static [u8] {
    let buffer = pybytes_as_mut_u8(op);
    let length = Py_SIZE(op) as usize;
    std::slice::from_raw_parts(buffer, length)
}

#[inline(always)]
pub unsafe fn pybytearray_as_bytes(op: *mut PyObject) -> &'static [u8] {
    let buffer = PyByteArray_AsString(op).cast::<u8>();
    let length = PyByteArray_Size(op) as usize;
    std::slice::from_raw_parts(buffer, length)
}

#[repr(C)]
#[cfg(not(PyPy))]
pub struct _PyManagedBufferObject {
    pub ob_base: *mut PyObject,
    pub flags: c_int,
    pub exports: Py_ssize_t,
    pub master: *mut Py_buffer,
}

#[repr(C)]
#[cfg(not(PyPy))]
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

#[repr(C)]
#[cfg(PyPy)]
pub struct PyMemoryViewObject {
    pub ob_base: PyObject,
    pub view: Py_buffer,
}

#[inline(always)]
pub unsafe fn pymemoryview_as_bytes(op: *mut PyObject) -> Option<&'static [u8]> {
    let view = &(*op.cast::<PyMemoryViewObject>()).view;
    if PyBuffer_IsContiguous(view, b'C' as c_char) == 0 {
        None
    } else {
        let buffer = view.buf.cast::<u8>();
        let length = view.len as usize;
        Some(std::slice::from_raw_parts(buffer, length))
    }
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
        unsafe {
            if PyDict_Next(self.op, &mut self.pos, &mut key, &mut value) == 1 {
                Some((NonNull::new_unchecked(key), NonNull::new_unchecked(value)))
            } else {
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = unsafe { pydict_size(self.op) } as usize;
        (len, Some(len))
    }
}
