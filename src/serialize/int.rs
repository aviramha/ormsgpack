// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::pylong_is_positive;
use serde::ser::{Serialize, Serializer};

// https://tools.ietf.org/html/rfc7159#section-6
// "[-(2**53)+1, (2**53)-1]"

#[repr(transparent)]
pub struct Int {
    ptr: *mut pyo3::ffi::PyObject,
}

impl Int {
    pub fn new(ptr: *mut pyo3::ffi::PyObject) -> Self {
        Int { ptr: ptr }
    }
}

impl Serialize for Int {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if pylong_is_positive(self.ptr) {
            let val = ffi!(PyLong_AsUnsignedLongLong(self.ptr));
            if unlikely!(val == u64::MAX && !ffi!(PyErr_Occurred()).is_null()) {
                err!("Integer exceeds 64-bit range")
            }
            serializer.serialize_u64(val)
        } else {
            let val = ffi!(PyLong_AsLongLong(self.ptr));
            if unlikely!(val == -1 && !ffi!(PyErr_Occurred()).is_null()) {
                err!("Integer exceeds 64-bit range")
            }
            serializer.serialize_i64(val)
        }
    }
}
