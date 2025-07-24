// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::*;
use ahash::RandomState;
use simdutf8::basic::{from_utf8, Utf8Error};
use std::hash::BuildHasher;
use std::hash::Hasher;
use std::ptr::NonNull;
#[cfg(Py_GIL_DISABLED)]
use std::sync::Mutex;

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
        unsafe { pyo3::ffi::Py_INCREF(self.ptr) };
        self.ptr
    }
}

impl Drop for CachedKey {
    fn drop(&mut self) {
        unsafe { pyo3::ffi::Py_DECREF(self.ptr) };
    }
}

pub struct KeyMap<const C: usize> {
    #[cfg(Py_GIL_DISABLED)]
    entries: Mutex<Vec<Option<CachedKey>>>,
    #[cfg(not(Py_GIL_DISABLED))]
    entries: Vec<Option<CachedKey>>,
    hash_builder: RandomState,
}

impl<const C: usize> KeyMap<C> {
    pub fn new() -> Self {
        let mut entries = Vec::with_capacity(C);
        for _ in 0..C {
            entries.push(None);
        }
        KeyMap {
            #[cfg(Py_GIL_DISABLED)]
            entries: Mutex::new(entries),
            #[cfg(not(Py_GIL_DISABLED))]
            entries: entries,
            hash_builder: RandomState::new(),
        }
    }

    pub fn get(&mut self, key: &[u8]) -> Result<NonNull<pyo3::ffi::PyObject>, Utf8Error> {
        let hash = {
            let mut hasher = self.hash_builder.build_hasher();
            hasher.write(key);
            hasher.finish()
        } as usize;
        let index = hash % C;
        #[cfg(Py_GIL_DISABLED)]
        let mut entries = self.entries.lock().unwrap();
        #[cfg(not(Py_GIL_DISABLED))]
        let entries = &mut self.entries;
        let entry = match &mut entries[index] {
            Some(v) if unicode_to_str(v.ptr).unwrap().as_bytes() == key => v,
            _ => {
                let pykey = unicode_from_str(from_utf8(key)?);
                hash_str(pykey);
                entries[index] = Some(CachedKey::new(pykey));
                match &mut entries[index] {
                    Some(v) => v,
                    _ => unreachable!(),
                }
            }
        };
        unsafe { Ok(NonNull::new_unchecked(entry.get())) }
    }
}
