// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::deserialize::cache::*;
use crate::deserialize::DeserializeError;
use crate::exc::*;
use crate::ffi::*;
use crate::opt::*;
use crate::typeref::*;
use crate::unicode::*;
use byteorder::{BigEndian, ReadBytesExt};
use rmp::Marker;
use simdutf8::basic::{from_utf8, Utf8Error};
use std::borrow::Cow;
use std::os::raw::c_char;
use std::ptr::NonNull;

const RECURSION_LIMIT: u16 = 1024;

pub fn deserialize(
    ptr: *mut pyo3::ffi::PyObject,
    ext_hook: Option<NonNull<pyo3::ffi::PyObject>>,
    opts: Opt,
) -> Result<NonNull<pyo3::ffi::PyObject>, DeserializeError<'static>> {
    let obj_type_ptr = ob_type!(ptr);
    let buffer: *const u8;
    let length: usize;

    if is_type!(obj_type_ptr, BYTES_TYPE) {
        buffer = unsafe { PyBytes_AS_STRING(ptr) as *const u8 };
        length = unsafe { PyBytes_GET_SIZE(ptr) as usize };
    } else if is_type!(obj_type_ptr, MEMORYVIEW_TYPE) {
        let membuf = unsafe { PyMemoryView_GET_BUFFER(ptr) };
        if unsafe { pyo3::ffi::PyBuffer_IsContiguous(membuf, b'C' as c_char) == 0 } {
            return Err(DeserializeError::new(Cow::Borrowed(
                "Input type memoryview must be a C contiguous buffer",
            )));
        }
        buffer = unsafe { (*membuf).buf as *const u8 };
        length = unsafe { (*membuf).len as usize };
    } else if is_type!(obj_type_ptr, BYTEARRAY_TYPE) {
        buffer = ffi!(PyByteArray_AsString(ptr)) as *const u8;
        length = ffi!(PyByteArray_Size(ptr)) as usize;
    } else {
        return Err(DeserializeError::new(Cow::Borrowed(
            "Input must be bytes, bytearray, memoryview",
        )));
    }
    let contents: &[u8] = unsafe { std::slice::from_raw_parts(buffer, length) };

    let mut deserializer = Deserializer::new(contents, ext_hook, opts);
    deserializer
        .deserialize()
        .map_err(|e| DeserializeError::new(Cow::Owned(e.to_string())))
}

#[derive(Debug)]
enum Error {
    ExtHookFailed,
    ExtHookMissing,
    Internal,
    InvalidStr,
    InvalidType(Marker),
    RecursionLimitReached,
    UnexpectedEof,
}

impl std::fmt::Display for Error {
    #[cold]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ExtHookFailed => f.write_str("ext_hook failed"),
            Error::ExtHookMissing => f.write_str("ext_hook missing"),
            Error::Internal => f.write_str("internal error"),
            Error::InvalidStr => f.write_str(INVALID_STR),
            Error::InvalidType(ref marker) => {
                write!(f, "invalid type {marker:?}")
            }
            Error::RecursionLimitReached => f.write_str(RECURSION_LIMIT_REACHED),
            Error::UnexpectedEof => write!(f, "unexpected end of file"),
        }
    }
}

impl From<Utf8Error> for Error {
    #[cold]
    fn from(_: Utf8Error) -> Error {
        Error::InvalidStr
    }
}

struct Deserializer<'de> {
    data: &'de [u8],
    ext_hook: Option<NonNull<pyo3::ffi::PyObject>>,
    opts: Opt,
    recursion: u16,
}

impl<'de> Deserializer<'de> {
    fn new(data: &'de [u8], ext_hook: Option<NonNull<pyo3::ffi::PyObject>>, opts: Opt) -> Self {
        Deserializer {
            data: data,
            ext_hook: ext_hook,
            opts: opts,
            recursion: 0,
        }
    }

    fn read_slice(&mut self, len: usize) -> Result<&'de [u8], Error> {
        if len > self.data.len() {
            return Err(Error::UnexpectedEof);
        }
        let (a, b) = self.data.split_at(len);
        self.data = b;
        Ok(a)
    }

    #[inline(always)]
    fn read_f32(&mut self) -> Result<f32, Error> {
        self.data
            .read_f32::<BigEndian>()
            .map_err(|_| Error::UnexpectedEof)
    }

    #[inline(always)]
    fn read_f64(&mut self) -> Result<f64, Error> {
        self.data
            .read_f64::<BigEndian>()
            .map_err(|_| Error::UnexpectedEof)
    }

    #[inline(always)]
    fn read_i8(&mut self) -> Result<i8, Error> {
        self.data.read_i8().map_err(|_| Error::UnexpectedEof)
    }

    #[inline(always)]
    fn read_i16(&mut self) -> Result<i16, Error> {
        self.data
            .read_i16::<BigEndian>()
            .map_err(|_| Error::UnexpectedEof)
    }

    #[inline(always)]
    fn read_i32(&mut self) -> Result<i32, Error> {
        self.data
            .read_i32::<BigEndian>()
            .map_err(|_| Error::UnexpectedEof)
    }

    #[inline(always)]
    fn read_i64(&mut self) -> Result<i64, Error> {
        self.data
            .read_i64::<BigEndian>()
            .map_err(|_| Error::UnexpectedEof)
    }

    #[inline(always)]
    fn read_u8(&mut self) -> Result<u8, Error> {
        self.data.read_u8().map_err(|_| Error::UnexpectedEof)
    }

    #[inline(always)]
    fn read_u16(&mut self) -> Result<u16, Error> {
        self.data
            .read_u16::<BigEndian>()
            .map_err(|_| Error::UnexpectedEof)
    }

    #[inline(always)]
    fn read_u32(&mut self) -> Result<u32, Error> {
        self.data
            .read_u32::<BigEndian>()
            .map_err(|_| Error::UnexpectedEof)
    }

    #[inline(always)]
    fn read_u64(&mut self) -> Result<u64, Error> {
        self.data
            .read_u64::<BigEndian>()
            .map_err(|_| Error::UnexpectedEof)
    }

    #[inline(always)]
    fn read_marker(&mut self) -> Result<Marker, Error> {
        let n = self.read_u8()?;
        Ok(Marker::from_u8(n))
    }

    fn deserialize_ext(&mut self, len: u32) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        let tag = self.read_i8()?;
        let data = self.read_slice(len as usize)?;

        match self.ext_hook {
            Some(callable) => {
                let tag_obj = ffi!(PyLong_FromLongLong(tag as i64));
                let data_ptr = data.as_ptr() as *const c_char;
                let data_len = data.len() as pyo3::ffi::Py_ssize_t;
                let data_obj = ffi!(PyBytes_FromStringAndSize(data_ptr, data_len));
                #[allow(clippy::unnecessary_cast)]
                let obj = ffi!(PyObject_CallFunctionObjArgs(
                    callable.as_ptr(),
                    tag_obj,
                    data_obj,
                    std::ptr::null_mut() as *mut pyo3::ffi::PyObject
                ));
                ffi!(Py_DECREF(tag_obj));
                ffi!(Py_DECREF(data_obj));
                if unlikely!(obj.is_null()) {
                    Err(Error::ExtHookFailed)
                } else {
                    Ok(nonnull!(obj))
                }
            }
            None => Err(Error::ExtHookMissing),
        }
    }

    fn deserialize_null(&self) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        ffi!(Py_INCREF(NONE));
        Ok(nonnull!(NONE))
    }

    fn deserialize_true(&self) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        ffi!(Py_INCREF(TRUE));
        Ok(nonnull!(TRUE))
    }

    fn deserialize_false(&self) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        ffi!(Py_INCREF(FALSE));
        Ok(nonnull!(FALSE))
    }

    fn deserialize_i64(&self, value: i64) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        Ok(nonnull!(ffi!(PyLong_FromLongLong(value))))
    }

    fn deserialize_u64(&self, value: u64) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        Ok(nonnull!(ffi!(PyLong_FromUnsignedLongLong(value))))
    }

    fn deserialize_f64(&self, value: f64) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        Ok(nonnull!(ffi!(PyFloat_FromDouble(value))))
    }

    fn deserialize_str(&mut self, len: u32) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        let data = self.read_slice(len as usize)?;
        let value = from_utf8(data)?;
        Ok(nonnull!(unicode_from_str(value)))
    }

    fn deserialize_bin(&mut self, len: u32) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        let v = self.read_slice(len as usize)?;
        let ptr = v.as_ptr() as *const c_char;
        let len = v.len() as pyo3::ffi::Py_ssize_t;
        Ok(nonnull!(ffi!(PyBytes_FromStringAndSize(ptr, len))))
    }

    fn deserialize_array(&mut self, len: u32) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        let ptr = ffi!(PyList_New(len as pyo3::ffi::Py_ssize_t));
        for i in 0..len {
            let elem = self.deserialize()?;
            ffi!(PyList_SET_ITEM(
                ptr,
                i as pyo3::ffi::Py_ssize_t,
                elem.as_ptr()
            ));
        }
        Ok(nonnull!(ptr))
    }

    fn deserialize_map_with_str_keys(
        &mut self,
        len: u32,
    ) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        let dict_ptr = ffi!(_PyDict_NewPresized(len as pyo3::ffi::Py_ssize_t));
        for _ in 0..len {
            let marker = self.read_marker()?;
            let key = match marker {
                Marker::FixStr(len) => self.deserialize_map_str_key(len.into()),
                Marker::Str8 => {
                    let len = self.read_u8()?;
                    self.deserialize_map_str_key(len.into())
                }
                Marker::Str16 => {
                    let len = self.read_u16()?;
                    self.deserialize_map_str_key(len.into())
                }
                Marker::Str32 => {
                    let len = self.read_u32()?;
                    self.deserialize_map_str_key(len)
                }
                marker => Err(Error::InvalidType(marker)),
            }?;
            let value = self.deserialize()?;
            let pyhash = unsafe { (*key.as_ptr().cast::<pyo3::ffi::PyASCIIObject>()).hash };
            let _ = ffi!(_PyDict_SetItem_KnownHash(
                dict_ptr,
                key.as_ptr(),
                value.as_ptr(),
                pyhash
            ));
            // counter Py_INCREF in insertdict
            ffi!(Py_DECREF(key.as_ptr()));
            ffi!(Py_DECREF(value.as_ptr()));
        }
        Ok(nonnull!(dict_ptr))
    }

    fn deserialize_map_with_non_str_keys(
        &mut self,
        len: u32,
    ) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        let dict_ptr = ffi!(_PyDict_NewPresized(len as pyo3::ffi::Py_ssize_t));
        for _ in 0..len {
            let key = self.deserialize_map_key()?;
            let value = self.deserialize()?;
            let ret = ffi!(PyDict_SetItem(dict_ptr, key.as_ptr(), value.as_ptr()));
            if unlikely!(ret == -1) {
                return Err(Error::Internal);
            }
            ffi!(Py_DECREF(key.as_ptr()));
            ffi!(Py_DECREF(value.as_ptr()));
        }
        Ok(nonnull!(dict_ptr))
    }

    fn deserialize_map(&mut self, len: u32) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        if self.opts & NON_STR_KEYS != 0 {
            self.deserialize_map_with_non_str_keys(len)
        } else {
            self.deserialize_map_with_str_keys(len)
        }
    }

    fn deserialize(&mut self) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        self.recursion += 1;
        if unlikely!(self.recursion == RECURSION_LIMIT) {
            return Err(Error::RecursionLimitReached);
        }

        let marker = self.read_marker()?;
        let value = match marker {
            Marker::Null => self.deserialize_null(),
            Marker::True => self.deserialize_true(),
            Marker::False => self.deserialize_false(),
            Marker::FixPos(value) => self.deserialize_u64(value.into()),
            Marker::U8 => {
                let value = self.read_u8()?;
                self.deserialize_u64(value.into())
            }
            Marker::U16 => {
                let value = self.read_u16()?;
                self.deserialize_u64(value.into())
            }
            Marker::U32 => {
                let value = self.read_u32()?;
                self.deserialize_u64(value.into())
            }
            Marker::U64 => {
                let value = self.read_u64()?;
                self.deserialize_u64(value)
            }
            Marker::FixNeg(value) => self.deserialize_i64(value.into()),
            Marker::I8 => {
                let value = self.read_i8()?;
                self.deserialize_i64(value.into())
            }
            Marker::I16 => {
                let value = self.read_i16()?;
                self.deserialize_i64(value.into())
            }
            Marker::I32 => {
                let value = self.read_i32()?;
                self.deserialize_i64(value.into())
            }
            Marker::I64 => {
                let value = self.read_i64()?;
                self.deserialize_i64(value)
            }
            Marker::F32 => {
                let value = self.read_f32()?;
                self.deserialize_f64(value.into())
            }
            Marker::F64 => {
                let value = self.read_f64()?;
                self.deserialize_f64(value)
            }
            Marker::FixStr(len) => self.deserialize_str(len.into()),
            Marker::Str8 => {
                let len = self.read_u8()?;
                self.deserialize_str(len.into())
            }
            Marker::Str16 => {
                let len = self.read_u16()?;
                self.deserialize_str(len.into())
            }
            Marker::Str32 => {
                let len = self.read_u32()?;
                self.deserialize_str(len)
            }
            Marker::Bin8 => {
                let len = self.read_u8()?;
                self.deserialize_bin(len.into())
            }
            Marker::Bin16 => {
                let len = self.read_u16()?;
                self.deserialize_bin(len.into())
            }
            Marker::Bin32 => {
                let len = self.read_u32()?;
                self.deserialize_bin(len)
            }
            Marker::FixArray(len) => self.deserialize_array(len.into()),
            Marker::Array16 => {
                let len = self.read_u16()?;
                self.deserialize_array(len.into())
            }
            Marker::Array32 => {
                let len = self.read_u32()?;
                self.deserialize_array(len)
            }
            Marker::FixMap(len) => self.deserialize_map(len.into()),
            Marker::Map16 => {
                let len = self.read_u16()?;
                self.deserialize_map(len.into())
            }
            Marker::Map32 => {
                let len = self.read_u32()?;
                self.deserialize_map(len)
            }
            Marker::FixExt1 => self.deserialize_ext(1),
            Marker::FixExt2 => self.deserialize_ext(2),
            Marker::FixExt4 => self.deserialize_ext(4),
            Marker::FixExt8 => self.deserialize_ext(8),
            Marker::FixExt16 => self.deserialize_ext(16),
            Marker::Ext8 => {
                let len = self.read_u8()?;
                self.deserialize_ext(len.into())
            }
            Marker::Ext16 => {
                let len = self.read_u16()?;
                self.deserialize_ext(len.into())
            }
            Marker::Ext32 => {
                let len = self.read_u32()?;
                self.deserialize_ext(len)
            }
            Marker::Reserved => Err(Error::InvalidType(Marker::Reserved)),
        };

        self.recursion -= 1;
        value
    }

    fn deserialize_map_str_key(&mut self, len: u32) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        if unlikely!(len > 64) {
            let value = self.deserialize_str(len)?;
            hash_str(value.as_ptr());
            Ok(value)
        } else {
            let data = self.read_slice(len as usize)?;
            let map = unsafe { KEY_MAP.get_mut().unwrap_or_else(|| unreachable!()) };
            Ok(map.get(data)?)
        }
    }

    fn deserialize_map_array_key(
        &mut self,
        len: u32,
    ) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        let ptr = ffi!(PyTuple_New(len as pyo3::ffi::Py_ssize_t));
        for i in 0..len {
            let elem = self.deserialize_map_key()?;
            ffi!(PyTuple_SET_ITEM(
                ptr,
                i as pyo3::ffi::Py_ssize_t,
                elem.as_ptr()
            ));
        }
        Ok(nonnull!(ptr))
    }

    fn deserialize_map_key(&mut self) -> Result<NonNull<pyo3::ffi::PyObject>, Error> {
        self.recursion += 1;
        if unlikely!(self.recursion == RECURSION_LIMIT) {
            return Err(Error::RecursionLimitReached);
        }

        let marker = self.read_marker()?;
        let value = match marker {
            Marker::Null => self.deserialize_null(),
            Marker::True => self.deserialize_true(),
            Marker::False => self.deserialize_false(),
            Marker::FixPos(value) => self.deserialize_u64(value.into()),
            Marker::U8 => {
                let value = self.read_u8()?;
                self.deserialize_u64(value.into())
            }
            Marker::U16 => {
                let value = self.read_u16()?;
                self.deserialize_u64(value.into())
            }
            Marker::U32 => {
                let value = self.read_u32()?;
                self.deserialize_u64(value.into())
            }
            Marker::U64 => {
                let value = self.read_u64()?;
                self.deserialize_u64(value)
            }
            Marker::FixNeg(value) => self.deserialize_i64(value.into()),
            Marker::I8 => {
                let value = self.read_i8()?;
                self.deserialize_i64(value.into())
            }
            Marker::I16 => {
                let value = self.read_i16()?;
                self.deserialize_i64(value.into())
            }
            Marker::I32 => {
                let value = self.read_i32()?;
                self.deserialize_i64(value.into())
            }
            Marker::I64 => {
                let value = self.read_i64()?;
                self.deserialize_i64(value)
            }
            Marker::F32 => {
                let value = self.read_f32()?;
                self.deserialize_f64(value.into())
            }
            Marker::F64 => {
                let value = self.read_f64()?;
                self.deserialize_f64(value)
            }
            Marker::FixStr(len) => self.deserialize_map_str_key(len.into()),
            Marker::Str8 => {
                let len = self.read_u8()?;
                self.deserialize_map_str_key(len.into())
            }
            Marker::Str16 => {
                let len = self.read_u16()?;
                self.deserialize_map_str_key(len.into())
            }
            Marker::Str32 => {
                let len = self.read_u32()?;
                self.deserialize_map_str_key(len)
            }
            Marker::Bin8 => {
                let len = self.read_u8()?;
                self.deserialize_bin(len.into())
            }
            Marker::Bin16 => {
                let len = self.read_u16()?;
                self.deserialize_bin(len.into())
            }
            Marker::Bin32 => {
                let len = self.read_u32()?;
                self.deserialize_bin(len)
            }
            Marker::FixArray(len) => self.deserialize_map_array_key(len.into()),
            Marker::Array16 => {
                let len = self.read_u16()?;
                self.deserialize_map_array_key(len.into())
            }
            Marker::Array32 => {
                let len = self.read_u32()?;
                self.deserialize_map_array_key(len)
            }
            marker => Err(Error::InvalidType(marker)),
        };

        self.recursion -= 1;
        value
    }
}
