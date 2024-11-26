// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::typeref::*;
use crate::unicode::*;
use once_cell::unsync::OnceCell;
use simdutf8::basic::{from_utf8, Utf8Error};
use std::hash::BuildHasher;
use std::hash::Hasher;
use std::ptr::NonNull;

#[repr(transparent)]
struct CachedKey {
    ptr: *mut pyo3::ffi::PyObject,
}

unsafe impl Send for CachedKey {}
unsafe impl Sync for CachedKey {}

impl CachedKey {
    fn new(ptr: *mut pyo3::ffi::PyObject) -> CachedKey {
        CachedKey { ptr: ptr }
    }

    fn get(&mut self) -> *mut pyo3::ffi::PyObject {
        ffi!(Py_INCREF(self.ptr));
        self.ptr
    }
}

impl Drop for CachedKey {
    fn drop(&mut self) {
        ffi!(Py_DECREF(self.ptr));
    }
}

pub struct KeyMap<const C: usize> {
    entries: Vec<Option<CachedKey>>,
}

impl<const C: usize> KeyMap<C> {
    pub fn new() -> Self {
        let mut entries = Vec::with_capacity(C);
        for _ in 0..C {
            entries.push(None);
        }
        KeyMap { entries: entries }
    }

    pub fn get(&mut self, key: &[u8]) -> Result<NonNull<pyo3::ffi::PyObject>, Utf8Error> {
        let mut hasher = unsafe { HASH_BUILDER.get().unwrap().build_hasher() };
        let hash = {
            hasher.write(key);
            hasher.finish()
        } as usize;
        let index = hash % C;
        let entry = match &mut self.entries[index] {
            Some(v) if unicode_to_str(v.ptr).unwrap().as_bytes() == key => v,
            _ => {
                let pykey = unicode_from_str(from_utf8(key)?);
                hash_str(pykey);
                self.entries[index] = Some(CachedKey::new(pykey));
                match &mut self.entries[index] {
                    Some(v) => v,
                    _ => unreachable!(),
                }
            }
        };
        Ok(nonnull!(entry.get()))
    }
}

pub static mut KEY_MAP: OnceCell<KeyMap<512>> = OnceCell::new();
