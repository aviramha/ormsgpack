// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::exc::*;
use crate::ffi::PyDictIter;
use crate::opt::*;
use crate::serialize::serializer::*;
use crate::typeref::*;
use crate::unicode::*;
use serde::ser::{Serialize, SerializeMap, Serializer};
use smallvec::SmallVec;
use std::ptr::NonNull;

pub struct Dict {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl Dict {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        Dict {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for Dict {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if unlikely!(ffi!(Py_SIZE(self.ptr)) == 0) {
            serializer.serialize_map(Some(0)).unwrap().end()
        } else if self.opts & (NON_STR_KEYS | SORT_KEYS) == 0 {
            DictWithStrKeys::new(
                self.ptr,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer)
        } else if self.opts & NON_STR_KEYS != 0 {
            if self.opts & SORT_KEYS != 0 {
                err!("OPT_NON_STR_KEYS is not compatible with OPT_SORT_KEYS")
            }
            DictWithNonStrKeys::new(
                self.ptr,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer)
        } else {
            DictWithSortedStrKeys::new(
                self.ptr,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer)
        }
    }
}

pub struct DictWithStrKeys {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl DictWithStrKeys {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        DictWithStrKeys {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for DictWithStrKeys {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = ffi!(Py_SIZE(self.ptr)) as usize;
        let mut map = serializer.serialize_map(Some(len)).unwrap();
        for (key, value) in PyDictIter::from_pyobject(self.ptr) {
            if unlikely!(!py_is!(ob_type!(key.as_ptr()), STR_TYPE)) {
                err!(KEY_MUST_BE_STR)
            }
            let data = unicode_to_str(key.as_ptr());
            if unlikely!(data.is_none()) {
                err!(INVALID_STR)
            }
            let pyvalue = PyObject::new(
                value.as_ptr(),
                self.opts,
                self.default_calls,
                self.recursion + 1,
                self.default,
            );
            map.serialize_key(data.unwrap()).unwrap();
            map.serialize_value(&pyvalue)?;
        }
        map.end()
    }
}

pub struct DictWithSortedStrKeys {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl DictWithSortedStrKeys {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        DictWithSortedStrKeys {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for DictWithSortedStrKeys {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = ffi!(Py_SIZE(self.ptr)) as usize;
        let mut items: SmallVec<[(&str, *mut pyo3::ffi::PyObject); 8]> =
            SmallVec::with_capacity(len);
        for (key, value) in PyDictIter::from_pyobject(self.ptr) {
            if unlikely!(!py_is!(ob_type!(key.as_ptr()), STR_TYPE)) {
                err!(KEY_MUST_BE_STR)
            }
            let data = unicode_to_str(key.as_ptr());
            if unlikely!(data.is_none()) {
                err!(INVALID_STR)
            }
            items.push((data.unwrap(), value.as_ptr()));
        }

        items.sort_unstable_by(|a, b| a.0.cmp(b.0));

        let mut map = serializer.serialize_map(Some(len)).unwrap();
        for (key, val) in items.iter() {
            let pyvalue = PyObject::new(
                *val,
                self.opts,
                self.default_calls,
                self.recursion + 1,
                self.default,
            );
            map.serialize_key(key).unwrap();
            map.serialize_value(&pyvalue)?;
        }
        map.end()
    }
}

pub struct DictWithNonStrKeys {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl DictWithNonStrKeys {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        DictWithNonStrKeys {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for DictWithNonStrKeys {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let opts = self.opts & NOT_PASSTHROUGH;
        let len = ffi!(Py_SIZE(self.ptr)) as usize;
        let mut map = serializer.serialize_map(Some(len)).unwrap();
        for (key, value) in PyDictIter::from_pyobject(self.ptr) {
            if py_is!(ob_type!(key.as_ptr()), STR_TYPE) {
                let data = unicode_to_str(key.as_ptr());
                if unlikely!(data.is_none()) {
                    err!(INVALID_STR)
                }
                map.serialize_entry(
                    data.unwrap(),
                    &PyObject::new(
                        value.as_ptr(),
                        self.opts,
                        self.default_calls,
                        self.recursion + 1,
                        self.default,
                    ),
                )?;
            } else {
                map.serialize_entry(
                    &DictKey::new(key.as_ptr(), opts, self.recursion + 1),
                    &PyObject::new(
                        value.as_ptr(),
                        self.opts,
                        self.default_calls,
                        self.recursion + 1,
                        self.default,
                    ),
                )?;
            }
        }
        map.end()
    }
}
