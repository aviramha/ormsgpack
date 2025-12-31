// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::*;
use crate::io::WriteSlices;
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

    fn insert_slices<const N: usize>(&mut self, bufs: [&[u8]; N]) {
        let len: usize = bufs.iter().map(|b| b.len()).sum();
        let new_len = self.len + len;
        if new_len > self.cap {
            self.grow(new_len);
        }
        let mut ptr = self.buffer_ptr();
        for buf in bufs {
            unsafe {
                std::ptr::copy_nonoverlapping(buf.as_ptr(), ptr, buf.len());
                ptr = ptr.add(buf.len());
            };
        }
        self.len = new_len;
    }
}

impl std::io::Write for BytesWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.insert_slices([buf]);
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
        self.insert_slices([buf]);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

impl WriteSlices for BytesWriter {
    fn write_slices<const N: usize>(&mut self, bufs: [&[u8]; N]) -> Result<(), std::io::Error> {
        self.insert_slices(bufs);
        Ok(())
    }
}
