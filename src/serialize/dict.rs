// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::exc::*;
use crate::ffi::PyDictIter;
use crate::opt::*;
use crate::serialize::serializer::pyobject_to_obtype;
use crate::serialize::serializer::*;
use crate::typeref::*;
use crate::unicode::*;
use serde::ser::{Serialize, SerializeMap, Serializer};
use smallvec::SmallVec;
use std::ptr::NonNull;

pub struct DictGenericSerializer {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl DictGenericSerializer {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        DictGenericSerializer {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for DictGenericSerializer {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if unlikely!(ffi!(Py_SIZE(self.ptr)) == 0) {
            serializer.serialize_map(Some(0)).unwrap().end()
        } else if self.opts & (NON_STR_KEYS | SORT_KEYS) == 0 {
            Dict::new(
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
            DictNonStrKey::new(
                self.ptr,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer)
        } else {
            DictSortedKey::new(
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
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = ffi!(Py_SIZE(self.ptr)) as usize;
        let mut map = serializer.serialize_map(Some(len)).unwrap();
        for (key, value) in PyDictIter::from_pyobject(self.ptr) {
            if unlikely!(!is_type!(ob_type!(key.as_ptr()), STR_TYPE)) {
                err!(KEY_MUST_BE_STR)
            }
            let data = unicode_to_str(key.as_ptr());
            if unlikely!(data.is_none()) {
                err!(INVALID_STR)
            }
            let pyvalue = PyObjectSerializer::new(
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

pub struct DictSortedKey {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl DictSortedKey {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        DictSortedKey {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for DictSortedKey {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = ffi!(Py_SIZE(self.ptr)) as usize;
        let mut items: SmallVec<[(&str, *mut pyo3::ffi::PyObject); 8]> =
            SmallVec::with_capacity(len);
        for (key, value) in PyDictIter::from_pyobject(self.ptr) {
            if unlikely!(!is_type!(ob_type!(key.as_ptr()), STR_TYPE)) {
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
            let pyvalue = PyObjectSerializer::new(
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

pub struct DictNonStrKey {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl DictNonStrKey {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        DictNonStrKey {
            ptr: ptr,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for DictNonStrKey {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let opts = self.opts & NOT_PASSTHROUGH;
        let len = ffi!(Py_SIZE(self.ptr)) as usize;
        let mut map = serializer.serialize_map(Some(len)).unwrap();
        for (key, value) in PyDictIter::from_pyobject(self.ptr) {
            if is_type!(ob_type!(key.as_ptr()), STR_TYPE) {
                let data = unicode_to_str(key.as_ptr());
                if unlikely!(data.is_none()) {
                    err!(INVALID_STR)
                }
                map.serialize_entry(
                    data.unwrap(),
                    &PyObjectSerializer::new(
                        value.as_ptr(),
                        self.opts,
                        self.default_calls,
                        self.recursion + 1,
                        self.default,
                    ),
                )?;
            } else {
                match pyobject_to_obtype(key.as_ptr(), opts) {
                    ObType::NumpyScalar
                    | ObType::NumpyArray
                    | ObType::Dict
                    | ObType::List
                    | ObType::Dataclass
                    | ObType::Pydantic
                    | ObType::Unknown => {
                        err!("Dict key must a type serializable with OPT_NON_STR_KEYS")
                    }
                    _ => (),
                }
                map.serialize_entry(
                    &PyObjectSerializer::new(
                        key.as_ptr(),
                        opts,
                        self.default_calls,
                        self.recursion + 1,
                        self.default,
                    ),
                    &PyObjectSerializer::new(
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
