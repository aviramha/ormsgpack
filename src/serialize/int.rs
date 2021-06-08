// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use serde::ser::{Serialize, Serializer};

// https://tools.ietf.org/html/rfc7159#section-6
// "[-(2**53)+1, (2**53)-1]"

pub struct IntSerializer {
    ptr: *mut pyo3::ffi::PyObject,
}

impl IntSerializer {
    pub fn new(ptr: *mut pyo3::ffi::PyObject) -> Self {
        IntSerializer { ptr: ptr }
    }
}

impl<'p> Serialize for IntSerializer {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let val = ffi!(PyLong_AsLongLong(self.ptr));
        if unlikely!(val == -1) && !ffi!(PyErr_Occurred()).is_null() {
            return UIntSerializer::new(self.ptr).serialize(serializer);
        }
        serializer.serialize_i64(val)
    }
}

#[repr(transparent)]
pub struct UIntSerializer {
    ptr: *mut pyo3::ffi::PyObject,
}

impl UIntSerializer {
    pub fn new(ptr: *mut pyo3::ffi::PyObject) -> Self {
        UIntSerializer { ptr: ptr }
    }
}

impl<'p> Serialize for UIntSerializer {
    #[cold]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ffi!(PyErr_Clear());
        let val = ffi!(PyLong_AsUnsignedLongLong(self.ptr));
        if unlikely!(val == u64::MAX) && !ffi!(PyErr_Occurred()).is_null() {
            err!("Integer exceeds 64-bit range")
        }
        serializer.serialize_u64(val)
    }
}
