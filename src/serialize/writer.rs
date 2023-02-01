// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::PyBytesObject;
use core::ptr::NonNull;
use pyo3::ffi::*;

const BUFFER_LENGTH: usize = 1024;

pub struct BytesWriter {
    cap: usize,
    len: usize,
    bytes: *mut PyBytesObject,
}

impl BytesWriter {
    pub fn new() -> Self {
        let bytes_: *mut PyBytesObject;
        unsafe {
            // Allocates an uninitialized PyBytes object of length BUFFER_LENGTH on the heap
            bytes_ = PyBytes_FromStringAndSize(std::ptr::null_mut(), BUFFER_LENGTH as isize)
                as *mut PyBytesObject;
        }
        assert!(! bytes_.is_null() , "MemoryError: allocation failed in PyBytes_FromStringAndSize!");

        BytesWriter {
            cap: BUFFER_LENGTH,
            len: 0,
            bytes: bytes_,
        }
    }

    pub fn finish(&mut self) -> NonNull<PyObject> {
        unsafe {
            (*self.bytes.cast::<PyVarObject>()).ob_size = self.len as Py_ssize_t;
            self.resize(self.len as isize);
            NonNull::new_unchecked(self.bytes as *mut PyObject)
        }
    }

    #[inline]
    fn buffer_ptr(&self) -> *mut u8 {
        unsafe {
            (&(*self.bytes.cast::<PyBytesObject>()).ob_sval as *const [i8; 1] as *mut u8)
                .add(self.len)
        }
    }

    #[inline(always)]
    pub fn resize(&mut self, len: isize) {
        unsafe {
            let retval = _PyBytes_Resize(
                &mut self.bytes as *mut *mut PyBytesObject as *mut *mut PyObject,
                len as _,
            );
            assert_eq!(retval, 0, "MemoryError while deserializing a Bytes object!");
        }
    }

    #[cold]
    fn grow(&mut self, len: usize) {
        while len >= self.cap {
            self.cap *= 2;
        }
        self.resize(self.cap as isize);
    }
}

impl std::io::Write for BytesWriter {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
        let to_write = buf.len();
        let end_length = self.len + to_write;
        if unlikely!(end_length > self.cap) {
            self.grow(end_length);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(buf.as_ptr() as *const u8, self.buffer_ptr(), to_write);
        };
        self.len = end_length;
        Ok(to_write)
    }
    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> std::result::Result<(), std::io::Error> {
        let _ = self.write(buf);
        Ok(())
    }
    #[inline]
    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
}
