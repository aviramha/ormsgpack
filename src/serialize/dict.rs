// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::exc::*;
use crate::ffi::*;
use crate::opt::*;
use crate::serialize::serializer::*;
use crate::state::State;
use serde::ser::{Serialize, SerializeMap, Serializer};
use smallvec::SmallVec;
use std::ptr::NonNull;

pub struct Dict {
    ptr: *mut pyo3::ffi::PyObject,
    state: *mut State,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl Dict {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        state: *mut State,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        Dict {
            state: state,
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
        if unlikely!(unsafe { pydict_size(self.ptr) } == 0) {
            serializer.serialize_map(Some(0)).unwrap().end()
        } else if self.opts & (NON_STR_KEYS | SORT_KEYS) == 0 {
            self.serialize_with_str_keys(serializer)
        } else if self.opts & NON_STR_KEYS != 0 {
            if self.opts & SORT_KEYS != 0 {
                return Err(serde::ser::Error::custom(
                    "OPT_NON_STR_KEYS is not compatible with OPT_SORT_KEYS",
                ));
            }
            self.serialize_with_non_str_keys(serializer)
        } else {
            self.serialize_with_sorted_str_keys(serializer)
        }
    }
}

impl Dict {
    #[inline(always)]
    fn serialize_with_str_keys<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = unsafe { pydict_size(self.ptr) } as usize;
        let mut map = serializer.serialize_map(Some(len)).unwrap();
        for (key, value) in PyDictIter::from_pyobject(self.ptr) {
            if unlikely!(!py_is!(
                ob_type!(key.as_ptr()),
                &mut pyo3::ffi::PyUnicode_Type
            )) {
                return Err(serde::ser::Error::custom(KEY_MUST_BE_STR));
            }
            let data = unicode_to_str(key.as_ptr());
            if unlikely!(data.is_none()) {
                return Err(serde::ser::Error::custom(INVALID_STR));
            }
            let pyvalue = PyObject::new(
                value.as_ptr(),
                self.state,
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

    #[inline(never)]
    fn serialize_with_sorted_str_keys<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = unsafe { pydict_size(self.ptr) } as usize;
        let mut items: SmallVec<[(&str, *mut pyo3::ffi::PyObject); 8]> =
            SmallVec::with_capacity(len);
        for (key, value) in PyDictIter::from_pyobject(self.ptr) {
            if unlikely!(!py_is!(
                ob_type!(key.as_ptr()),
                &mut pyo3::ffi::PyUnicode_Type
            )) {
                return Err(serde::ser::Error::custom(KEY_MUST_BE_STR));
            }
            let data = unicode_to_str(key.as_ptr());
            if unlikely!(data.is_none()) {
                return Err(serde::ser::Error::custom(INVALID_STR));
            }
            items.push((data.unwrap(), value.as_ptr()));
        }

        items.sort_unstable_by(|a, b| a.0.cmp(b.0));

        let mut map = serializer.serialize_map(Some(len)).unwrap();
        for (key, val) in items.iter() {
            let pyvalue = PyObject::new(
                *val,
                self.state,
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

    #[inline(never)]
    fn serialize_with_non_str_keys<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let opts = self.opts & NOT_PASSTHROUGH;
        let len = unsafe { pydict_size(self.ptr) } as usize;
        let mut map = serializer.serialize_map(Some(len)).unwrap();
        for (key, value) in PyDictIter::from_pyobject(self.ptr) {
            if py_is!(ob_type!(key.as_ptr()), &mut pyo3::ffi::PyUnicode_Type) {
                let data = unicode_to_str(key.as_ptr());
                if unlikely!(data.is_none()) {
                    return Err(serde::ser::Error::custom(INVALID_STR));
                }
                map.serialize_entry(
                    data.unwrap(),
                    &PyObject::new(
                        value.as_ptr(),
                        self.state,
                        self.opts,
                        self.default_calls,
                        self.recursion + 1,
                        self.default,
                    ),
                )?;
            } else {
                map.serialize_entry(
                    &DictKey::new(key.as_ptr(), self.state, opts, self.recursion + 1),
                    &PyObject::new(
                        value.as_ptr(),
                        self.state,
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
