// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::ffi::*;

#[cfg(Py_GIL_DISABLED)]
pub struct CriticalSection(PyCriticalSection);

#[cfg(Py_GIL_DISABLED)]
impl CriticalSection {
    #[inline(always)]
    pub fn new() -> Self {
        CriticalSection(unsafe { std::mem::zeroed() })
    }

    #[inline(always)]
    pub fn begin(&mut self, op: *mut PyObject) {
        unsafe {
            PyCriticalSection_Begin(&mut self.0, op);
        }
    }
}

#[cfg(Py_GIL_DISABLED)]
impl Drop for CriticalSection {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            PyCriticalSection_End(&mut self.0);
        }
    }
}

#[cfg(not(Py_GIL_DISABLED))]
pub struct CriticalSection();

#[cfg(not(Py_GIL_DISABLED))]
impl CriticalSection {
    #[inline(always)]
    pub fn new() -> Self {
        CriticalSection()
    }

    #[inline(always)]
    pub fn begin(&mut self, _op: *mut PyObject) {}
}
