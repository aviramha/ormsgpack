// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::*;
use pyo3::ffi::*;
use std::ptr::NonNull;

const BUFFER_LENGTH: usize = 1024;

pub struct BytesWriter {
    cap: usize,
    len: usize,
    bytes: *mut PyObject,
}

impl BytesWriter {
    pub fn default() -> Self {
        BytesWriter {
            cap: BUFFER_LENGTH,
            len: 0,
            bytes: unsafe {
                PyBytes_FromStringAndSize(std::ptr::null_mut(), BUFFER_LENGTH as isize)
            },
        }
    }

    pub fn finish(&mut self) -> NonNull<PyObject> {
        unsafe {
            std::ptr::write(self.buffer_ptr(), 0);
            self.resize(self.len);
            NonNull::new_unchecked(self.bytes)
        }
    }

    fn buffer_ptr(&self) -> *mut u8 {
        unsafe { pybytes_as_mut_u8(self.bytes).add(self.len) }
    }

    #[inline]
    pub fn resize(&mut self, len: usize) {
        self.cap = len;
        unsafe {
            _PyBytes_Resize(&raw mut self.bytes, len as isize);
        }
    }

    #[cold]
    #[inline(never)]
    fn grow(&mut self, len: usize) {
        let mut cap = self.cap;
        while len >= cap {
            if len < 262144 {
                cap *= 4;
            } else {
                cap *= 2;
            }
        }
        self.resize(cap);
    }
}

impl std::io::Write for BytesWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        let _ = self.write_all(buf);
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
        let to_write = buf.len();
        let end_length = self.len + to_write;
        if unlikely!(end_length > self.cap) {
            self.grow(end_length);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(buf.as_ptr(), self.buffer_ptr(), to_write);
        };
        self.len = end_length;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}
