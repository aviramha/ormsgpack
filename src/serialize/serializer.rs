// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::exc::*;
use crate::ffi::*;
use crate::msgpack;
use crate::opt::*;
use crate::serialize::bytearray::*;
use crate::serialize::bytes::*;
use crate::serialize::dataclass::*;
use crate::serialize::datetime::*;
use crate::serialize::default::*;
use crate::serialize::dict::*;
use crate::serialize::ext::*;
use crate::serialize::list::*;
use crate::serialize::memoryview::*;
use crate::serialize::numpy::*;
use crate::serialize::pydantic::*;
use crate::serialize::str::*;
use crate::serialize::tuple::*;
use crate::serialize::uuid::*;
use crate::serialize::writer::*;
use crate::state::State;
use serde::ser::{Impossible, Serialize, SerializeMap, SerializeSeq, Serializer};
use std::os::raw::c_ulong;
use std::ptr::NonNull;

pub const RECURSION_LIMIT: u8 = 255;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    Write,
}

impl std::fmt::Display for Error {
    #[cold]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Custom(ref msg) => f.write_str(msg),
            Error::Write => f.write_str("write error"),
        }
    }
}

impl From<std::io::Error> for Error {
    #[cold]
    fn from(_: std::io::Error) -> Error {
        Error::Write
    }
}

impl serde::ser::Error for Error {
    #[cold]
    fn custom<T>(msg: T) -> Error
    where
        T: std::fmt::Display,
    {
        Error::Custom(msg.to_string())
    }
}

impl std::error::Error for Error {}

struct ExtSerializer<'a, W> {
    tag: i8,
    writer: &'a mut W,
}

impl<'a, W> ExtSerializer<'a, W>
where
    W: std::io::Write,
{
    #[inline]
    fn new(tag: i8, writer: &'a mut W) -> Self {
        Self {
            tag: tag,
            writer: writer,
        }
    }
}

impl<W> Serializer for &mut ExtSerializer<'_, W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        msgpack::write_ext(self.writer, value, self.tag)?;
        Ok(())
    }

    fn serialize_bool(self, _value: bool) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_i8(self, _value: i8) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_i16(self, _value: i16) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_i32(self, _value: i32) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_i64(self, _value: i64) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_u8(self, _value: u8) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_u16(self, _value: u16) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_u32(self, _value: u32) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_u64(self, _value: u64) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_char(self, _value: char) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_str(self, _value: &str) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unreachable!();
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unreachable!();
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unreachable!();
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unreachable!();
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> {
        unreachable!();
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        unreachable!();
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        unreachable!();
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        unreachable!();
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        unreachable!();
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        unreachable!();
    }
}

pub struct MessagePackSerializer<W> {
    writer: W,
}

impl<W> MessagePackSerializer<W>
where
    W: std::io::Write,
{
    #[inline]
    pub fn new(writer: W) -> Self {
        MessagePackSerializer { writer }
    }
}

pub struct Compound<'a, W> {
    se: &'a mut MessagePackSerializer<W>,
}

impl<W> SerializeSeq for Compound<'_, W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.se)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<W> SerializeMap for Compound<'_, W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut *self.se)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.se)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> Serializer for &'a mut MessagePackSerializer<W>
where
    W: std::io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        msgpack::write_bool(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(value))
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(value))
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(value))
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        msgpack::write_i64(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(&value.to_be_bytes())
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(value))
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(value))
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(value))
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        msgpack::write_u64(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(&value.to_be_bytes())
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        msgpack::write_f32(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        msgpack::write_f64(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_char(self, _value: char) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        msgpack::write_str(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        msgpack::write_bin(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        msgpack::write_nil(&mut self.writer)?;
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_unit_variant(
        self,
        _name: &str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unreachable!();
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let tag: i8 = match variant_index {
            128 => -1,
            _ => variant_index.try_into().unwrap_or_else(|_| unreachable!()),
        };
        let mut ext_se = ExtSerializer::new(tag, &mut self.writer);
        value.serialize(&mut ext_se)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        match len {
            Some(len) => {
                msgpack::write_array_len(&mut self.writer, len)?;
                Ok(Compound { se: self })
            }
            None => unreachable!(),
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unreachable!();
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unreachable!();
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        unreachable!();
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        match len {
            Some(len) => {
                msgpack::write_map_len(&mut self.writer, len)?;
                Ok(Compound { se: self })
            }
            None => unreachable!(),
        }
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        unreachable!();
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        unreachable!();
    }
}

pub fn serialize(
    ptr: *mut pyo3::ffi::PyObject,
    state: *mut State,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
    opts: Opt,
) -> Result<NonNull<pyo3::ffi::PyObject>, String> {
    let mut buf = BytesWriter::default();
    let obj = PyObject::new(ptr, state, opts, 0, 0, default);
    let mut ser = MessagePackSerializer::new(&mut buf);
    let res = obj.serialize(&mut ser);
    match res {
        Ok(_) => Ok(buf.finish()),
        Err(err) => {
            unsafe { pyo3::ffi::Py_DECREF(buf.finish().as_ptr()) };
            Err(err.to_string())
        }
    }
}

#[inline(always)]
fn is_subclass(op: *mut pyo3::ffi::PyTypeObject, feature: c_ulong) -> bool {
    unsafe { pyo3::ffi::PyType_HasFeature(op, feature) != 0 }
}

pub struct PyObject {
    ptr: *mut pyo3::ffi::PyObject,
    state: *mut State,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl PyObject {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        state: *mut State,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        PyObject {
            ptr: ptr,
            state: state,
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }

    #[inline(never)]
    fn serialize_unlikely<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ob_type = ob_type!(self.ptr);

        if self.opts & PASSTHROUGH_DATETIME == 0 {
            let datetime_api = unsafe { *pyo3::ffi::PyDateTimeAPI() };
            if ob_type == datetime_api.DateTimeType {
                match DateTime::new(self.ptr, self.state, self.opts) {
                    Ok(val) => return val.serialize(serializer),
                    Err(err) => return Err(serde::ser::Error::custom(err)),
                }
            }
            if ob_type == datetime_api.DateType {
                return Date::new(self.ptr).serialize(serializer);
            }
            if ob_type == datetime_api.TimeType {
                match Time::new(self.ptr, self.opts) {
                    Ok(val) => return val.serialize(serializer),
                    Err(err) => return Err(serde::ser::Error::custom(err)),
                };
            }
        }

        if self.opts & PASSTHROUGH_TUPLE == 0 && ob_type == &raw mut pyo3::ffi::PyTuple_Type {
            if unlikely!(self.recursion == RECURSION_LIMIT) {
                return Err(serde::ser::Error::custom(RECURSION_LIMIT_REACHED));
            }
            return Tuple::new(
                self.ptr,
                self.state,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer);
        }

        if self.opts & PASSTHROUGH_UUID == 0 && ob_type == unsafe { (*self.state).uuid_type } {
            return UUID::new(self.ptr, self.state).serialize(serializer);
        }

        if ob_type!(ob_type) == unsafe { (*self.state).enum_type } {
            if self.opts & PASSTHROUGH_ENUM == 0 {
                let value =
                    unsafe { pyo3::ffi::PyObject_GetAttr(self.ptr, (*self.state).value_str) };
                unsafe { pyo3::ffi::Py_DECREF(value) };
                return PyObject::new(
                    value,
                    self.state,
                    self.opts,
                    self.default_calls,
                    self.recursion,
                    self.default,
                )
                .serialize(serializer);
            } else {
                return Default::new(
                    self.ptr,
                    self.state,
                    self.opts,
                    self.default_calls,
                    self.recursion,
                    self.default,
                )
                .serialize(serializer);
            }
        }

        if self.opts & PASSTHROUGH_SUBCLASS == 0 {
            if is_subclass(ob_type, pyo3::ffi::Py_TPFLAGS_UNICODE_SUBCLASS) {
                return StrSubclass::new(self.ptr, self.opts).serialize(serializer);
            }
            if is_subclass(ob_type, pyo3::ffi::Py_TPFLAGS_LONG_SUBCLASS) {
                match Int::new(self.ptr) {
                    Ok(val) => return val.serialize(serializer),
                    Err(err) => {
                        if self.opts & PASSTHROUGH_BIG_INT != 0 {
                            return Default::new(
                                self.ptr,
                                self.state,
                                self.opts,
                                self.default_calls,
                                self.recursion,
                                self.default,
                            )
                            .serialize(serializer);
                        } else {
                            return Err(serde::ser::Error::custom(err));
                        }
                    }
                }
            }
            if is_subclass(ob_type, pyo3::ffi::Py_TPFLAGS_LIST_SUBCLASS) {
                if unlikely!(self.recursion == RECURSION_LIMIT) {
                    return Err(serde::ser::Error::custom(RECURSION_LIMIT_REACHED));
                }
                return List::new(
                    self.ptr,
                    self.state,
                    self.opts,
                    self.default_calls,
                    self.recursion,
                    self.default,
                )
                .serialize(serializer);
            }
            if is_subclass(ob_type, pyo3::ffi::Py_TPFLAGS_DICT_SUBCLASS) {
                if unlikely!(self.recursion == RECURSION_LIMIT) {
                    return Err(serde::ser::Error::custom(RECURSION_LIMIT_REACHED));
                }
                return Dict::new(
                    self.ptr,
                    self.state,
                    self.opts,
                    self.default_calls,
                    self.recursion,
                    self.default,
                )
                .serialize(serializer);
            }
        }

        if ob_type == unsafe { (*self.state).ext_type } {
            return Ext::new(self.ptr).serialize(serializer);
        }

        if self.opts & PASSTHROUGH_DATACLASS == 0 && is_dataclass(ob_type, self.state) {
            if unlikely!(self.recursion == RECURSION_LIMIT) {
                return Err(serde::ser::Error::custom(RECURSION_LIMIT_REACHED));
            }
            return Dataclass::new(
                self.ptr,
                self.state,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer);
        }

        if self.opts & SERIALIZE_PYDANTIC != 0 && is_pydantic_model(ob_type, self.state) {
            if unlikely!(self.recursion == RECURSION_LIMIT) {
                return Err(serde::ser::Error::custom(RECURSION_LIMIT_REACHED));
            }
            match PydanticModel::new(
                self.ptr,
                self.state,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            ) {
                Ok(val) => return val.serialize(serializer),
                Err(err) => return Err(serde::ser::Error::custom(err)),
            };
        }

        if self.opts & SERIALIZE_NUMPY != 0 {
            if let Some(numpy_types_ref) = unsafe { (*self.state).get_numpy_types() } {
                if ob_type == numpy_types_ref.bool_ {
                    return NumpyBool::new(self.ptr).serialize(serializer);
                }
                if ob_type == numpy_types_ref.datetime64 {
                    return NumpyDatetime64::new(self.ptr, self.state, self.opts)
                        .serialize(serializer);
                }
                if ob_type == numpy_types_ref.float16 {
                    return NumpyFloat16::new(self.ptr).serialize(serializer);
                }
                if ob_type == numpy_types_ref.float32 {
                    return NumpyFloat32::new(self.ptr).serialize(serializer);
                }
                if ob_type == numpy_types_ref.float64 {
                    return NumpyFloat64::new(self.ptr).serialize(serializer);
                }
                if ob_type == numpy_types_ref.int8 {
                    return NumpyInt8::new(self.ptr).serialize(serializer);
                }
                if ob_type == numpy_types_ref.int16 {
                    return NumpyInt16::new(self.ptr).serialize(serializer);
                }
                if ob_type == numpy_types_ref.int32 {
                    return NumpyInt32::new(self.ptr).serialize(serializer);
                }
                if ob_type == numpy_types_ref.int64 {
                    return NumpyInt64::new(self.ptr).serialize(serializer);
                }
                if ob_type == numpy_types_ref.uint8 {
                    return NumpyUint8::new(self.ptr).serialize(serializer);
                }
                if ob_type == numpy_types_ref.uint16 {
                    return NumpyUint16::new(self.ptr).serialize(serializer);
                }
                if ob_type == numpy_types_ref.uint32 {
                    return NumpyUint32::new(self.ptr).serialize(serializer);
                }
                if ob_type == numpy_types_ref.uint64 {
                    return NumpyUint64::new(self.ptr).serialize(serializer);
                }
                if ob_type == numpy_types_ref.array {
                    match NumpyArray::new(self.ptr, self.state, self.opts) {
                        Ok(val) => return val.serialize(serializer),
                        Err(PyArrayError::Malformed) => {
                            return Err(serde::ser::Error::custom("numpy array is malformed"))
                        }
                        Err(PyArrayError::NotContiguous)
                        | Err(PyArrayError::UnsupportedDataType) => {
                            if self.default.is_none() {
                                return Err(serde::ser::Error::custom("numpy array is not C contiguous; use ndarray.tolist() in default"));
                            }
                        }
                    }
                }
            }
        }

        if ob_type == &raw mut pyo3::ffi::PyByteArray_Type {
            return ByteArray::new(self.ptr).serialize(serializer);
        }
        if ob_type == &raw mut pyo3::ffi::PyMemoryView_Type {
            return MemoryView::new(self.ptr).serialize(serializer);
        }

        Default::new(
            self.ptr,
            self.state,
            self.opts,
            self.default_calls,
            self.recursion,
            self.default,
        )
        .serialize(serializer)
    }
}

impl Serialize for PyObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ob_type = ob_type!(self.ptr);
        if ob_type == &raw mut pyo3::ffi::PyUnicode_Type {
            Str::new(self.ptr, self.opts).serialize(serializer)
        } else if ob_type == &raw mut pyo3::ffi::PyBytes_Type {
            Bytes::new(self.ptr).serialize(serializer)
        } else if ob_type == &raw mut pyo3::ffi::PyLong_Type {
            match Int::new(self.ptr) {
                Ok(val) => val.serialize(serializer),
                Err(err) => {
                    if self.opts & PASSTHROUGH_BIG_INT != 0 {
                        Default::new(
                            self.ptr,
                            self.state,
                            self.opts,
                            self.default_calls,
                            self.recursion,
                            self.default,
                        )
                        .serialize(serializer)
                    } else {
                        Err(serde::ser::Error::custom(err))
                    }
                }
            }
        } else if ob_type == &raw mut pyo3::ffi::PyBool_Type {
            serializer.serialize_bool(unsafe { self.ptr == pyo3::ffi::Py_True() })
        } else if self.ptr == unsafe { pyo3::ffi::Py_None() } {
            serializer.serialize_unit()
        } else if ob_type == &raw mut pyo3::ffi::PyFloat_Type {
            serializer.serialize_f64(unsafe { pyo3::ffi::PyFloat_AS_DOUBLE(self.ptr) })
        } else if ob_type == &raw mut pyo3::ffi::PyList_Type {
            if unlikely!(self.recursion == RECURSION_LIMIT) {
                return Err(serde::ser::Error::custom(RECURSION_LIMIT_REACHED));
            }
            List::new(
                self.ptr,
                self.state,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer)
        } else if ob_type == &raw mut pyo3::ffi::PyDict_Type {
            if unlikely!(self.recursion == RECURSION_LIMIT) {
                return Err(serde::ser::Error::custom(RECURSION_LIMIT_REACHED));
            }
            Dict::new(
                self.ptr,
                self.state,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer)
        } else {
            self.serialize_unlikely(serializer)
        }
    }
}

pub struct DictTupleKey {
    ptr: *mut pyo3::ffi::PyObject,
    state: *mut State,
    opts: Opt,
    recursion: u8,
}

impl DictTupleKey {
    pub fn new(ptr: *mut pyo3::ffi::PyObject, state: *mut State, opts: Opt, recursion: u8) -> Self {
        DictTupleKey {
            ptr: ptr,
            state: state,
            opts: opts,
            recursion: recursion,
        }
    }
}

impl Serialize for DictTupleKey {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = unsafe { pyo3::ffi::Py_SIZE(self.ptr) } as usize;
        let mut seq = serializer.serialize_seq(Some(len)).unwrap();
        for i in 0..len {
            let item = unsafe { pytuple_get_item(self.ptr, i as isize) };
            let value = DictKey::new(item, self.state, self.opts, self.recursion + 1);
            seq.serialize_element(&value)?;
        }
        seq.end()
    }
}

pub struct DictKey {
    ptr: *mut pyo3::ffi::PyObject,
    state: *mut State,
    opts: Opt,
    recursion: u8,
}

impl DictKey {
    pub fn new(ptr: *mut pyo3::ffi::PyObject, state: *mut State, opts: Opt, recursion: u8) -> Self {
        DictKey {
            ptr: ptr,
            state: state,
            opts: opts,
            recursion: recursion,
        }
    }

    #[inline(never)]
    fn serialize_unlikely<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ob_type = ob_type!(self.ptr);

        let datetime_api = unsafe { *pyo3::ffi::PyDateTimeAPI() };
        if ob_type == datetime_api.DateTimeType {
            match DateTime::new(self.ptr, self.state, self.opts) {
                Ok(val) => return val.serialize(serializer),
                Err(err) => return Err(serde::ser::Error::custom(err)),
            }
        }
        if ob_type == datetime_api.DateType {
            return Date::new(self.ptr).serialize(serializer);
        }
        if ob_type == datetime_api.TimeType {
            match Time::new(self.ptr, self.opts) {
                Ok(val) => return val.serialize(serializer),
                Err(err) => return Err(serde::ser::Error::custom(err)),
            };
        }

        if ob_type == &raw mut pyo3::ffi::PyTuple_Type {
            if unlikely!(self.recursion == RECURSION_LIMIT) {
                return Err(serde::ser::Error::custom(RECURSION_LIMIT_REACHED));
            }
            return DictTupleKey::new(self.ptr, self.state, self.opts, self.recursion)
                .serialize(serializer);
        }

        if ob_type == unsafe { (*self.state).uuid_type } {
            return UUID::new(self.ptr, self.state).serialize(serializer);
        }

        if ob_type!(ob_type) == unsafe { (*self.state).enum_type } {
            let value = unsafe { pyo3::ffi::PyObject_GetAttr(self.ptr, (*self.state).value_str) };
            unsafe { pyo3::ffi::Py_DECREF(value) };
            return DictKey::new(value, self.state, self.opts, self.recursion)
                .serialize(serializer);
        }

        if is_subclass(ob_type, pyo3::ffi::Py_TPFLAGS_UNICODE_SUBCLASS) {
            return StrSubclass::new(self.ptr, self.opts).serialize(serializer);
        }
        if is_subclass(ob_type, pyo3::ffi::Py_TPFLAGS_LONG_SUBCLASS) {
            match Int::new(self.ptr) {
                Ok(val) => return val.serialize(serializer),
                Err(err) => return Err(serde::ser::Error::custom(err)),
            }
        }

        if ob_type == &raw mut pyo3::ffi::PyMemoryView_Type {
            return MemoryView::new(self.ptr).serialize(serializer);
        }

        Err(serde::ser::Error::custom(
            "Dict key must a type serializable with OPT_NON_STR_KEYS",
        ))
    }
}

impl Serialize for DictKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ob_type = ob_type!(self.ptr);
        if ob_type == &raw mut pyo3::ffi::PyUnicode_Type {
            Str::new(self.ptr, self.opts).serialize(serializer)
        } else if ob_type == &raw mut pyo3::ffi::PyBytes_Type {
            Bytes::new(self.ptr).serialize(serializer)
        } else if ob_type == &raw mut pyo3::ffi::PyLong_Type {
            match Int::new(self.ptr) {
                Ok(val) => val.serialize(serializer),
                Err(err) => Err(serde::ser::Error::custom(err)),
            }
        } else if ob_type == &raw mut pyo3::ffi::PyBool_Type {
            serializer.serialize_bool(unsafe { self.ptr == pyo3::ffi::Py_True() })
        } else if self.ptr == unsafe { pyo3::ffi::Py_None() } {
            serializer.serialize_unit()
        } else if ob_type == &raw mut pyo3::ffi::PyFloat_Type {
            serializer.serialize_f64(unsafe { pyo3::ffi::PyFloat_AS_DOUBLE(self.ptr) })
        } else {
            self.serialize_unlikely(serializer)
        }
    }
}
