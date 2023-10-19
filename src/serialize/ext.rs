// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ext::Ext;
use crate::ffi::*;

use serde::ser::{Serialize, Serializer};
use serde_bytes::ByteBuf;

#[repr(transparent)]
pub struct ExtSerializer {
    ptr: *mut pyo3::ffi::PyObject,
}

impl ExtSerializer {
    pub fn new(ptr: *mut pyo3::ffi::PyObject) -> Self {
        ExtSerializer { ptr: ptr }
    }
}

impl Serialize for ExtSerializer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ext = self.ptr as *mut Ext;
        let tag = ffi!(PyLong_AsLongLong((*ext).tag));
        if unlikely!(!(0..=127).contains(&tag)) {
            err!("Extension type out of range")
        }
        let buffer = unsafe { PyBytes_AS_STRING((*ext).data) as *const u8 };
        let length = unsafe { PyBytes_GET_SIZE((*ext).data) as usize };
        let data = unsafe { std::slice::from_raw_parts(buffer, length) };

        serializer.serialize_newtype_struct(
            rmp_serde::MSGPACK_EXT_STRUCT_NAME,
            &(tag as i8, ByteBuf::from(data)),
        )
    }
}
