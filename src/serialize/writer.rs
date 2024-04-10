// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::ffi::*;
use std::os::raw::c_char;
use std::ptr::NonNull;

const BUFFER_LENGTH: usize = 1024;

pub struct BytesWriter {
    cap: usize,
    len: usize,
    bytes: *mut PyBytesObject,
}

impl BytesWriter {
    pub fn default() -> Self {
        BytesWriter {
            cap: BUFFER_LENGTH,
            len: 0,
            bytes: unsafe {
                PyBytes_FromStringAndSize(std::ptr::null_mut(), BUFFER_LENGTH as isize)
                    as *mut PyBytesObject
            },
        }
    }

    pub fn finish(&mut self) -> NonNull<PyObject> {
        unsafe {
            std::ptr::write(self.buffer_ptr(), 0);
            (*self.bytes.cast::<PyVarObject>()).ob_size = self.len as Py_ssize_t;
            self.resize(self.len);
            NonNull::new_unchecked(self.bytes as *mut PyObject)
        }
    }

    fn buffer_ptr(&self) -> *mut u8 {
        unsafe {
            std::mem::transmute::<*mut [c_char; 1], *mut u8>(std::ptr::addr_of_mut!(
                (*self.bytes).ob_sval
            ))
            .add(self.len)
        }
    }

    #[inline]
    pub fn resize(&mut self, len: usize) {
        self.cap = len;
        unsafe {
            _PyBytes_Resize(
                std::ptr::addr_of_mut!(self.bytes) as *mut *mut PyObject,
                len as isize,
            );
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
