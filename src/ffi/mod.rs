// SPDX-License-Identifier: (Apache-2.0 OR MIT)

#[cfg_attr(any(PyPy, GraalPy), path = "base/mod.rs")]
#[cfg_attr(not(any(PyPy, GraalPy)), path = "cpython/mod.rs")]
mod impl_;
mod int;

pub use impl_::*;
pub use int::*;

use pyo3::ffi::*;
use std::os::raw::c_int;
use std::ptr::NonNull;

pub unsafe fn pybytes_as_u8(op: *mut PyObject) -> *const u8 {
    pybytes_as_mut_u8(op) as *const u8
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

#[allow(non_snake_case)]
#[inline(always)]
pub unsafe fn PyMemoryView_GET_BUFFER(op: *mut PyObject) -> *const Py_buffer {
    &(*op.cast::<PyMemoryViewObject>()).view
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
        let len = unsafe { pydict_size(self.op) } as usize;
        (len, Some(len))
    }
}
