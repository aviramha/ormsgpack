// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::exc::*;
use crate::ffi::*;
use crate::opt::*;
use crate::serialize::bytes::*;
use crate::serialize::dataclass::*;
use crate::serialize::datetime::*;
use crate::serialize::default::*;
use crate::serialize::dict::*;
use crate::serialize::ext::*;
use crate::serialize::int::*;
use crate::serialize::list::*;
use crate::serialize::numpy::*;
use crate::serialize::str::*;
use crate::serialize::tuple::*;
use crate::serialize::uuid::*;
use crate::serialize::writer::*;
use crate::typeref::*;
use serde::ser::{Impossible, Serialize, SerializeMap, SerializeSeq, Serializer};
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

impl From<rmp::encode::ValueWriteError> for Error {
    #[cold]
    fn from(_: rmp::encode::ValueWriteError) -> Error {
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
        rmp::encode::write_ext_meta(self.writer, value.len() as u32, self.tag)?;
        self.writer.write_all(value).map_err(|_| Error::Write)
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
        rmp::encode::write_bool(&mut self.writer, value).map_err(|_| Error::Write)
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
        rmp::encode::write_sint(&mut self.writer, value)?;
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
        rmp::encode::write_uint(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(&value.to_be_bytes())
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        rmp::encode::write_f32(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        rmp::encode::write_f64(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_char(self, _value: char) -> Result<Self::Ok, Self::Error> {
        unreachable!();
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        rmp::encode::write_str(&mut self.writer, value)?;
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        rmp::encode::write_bin(&mut self.writer, value)?;
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
        rmp::encode::write_nil(&mut self.writer).map_err(|_| Error::Write)
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
        let tag: i8 = variant_index.try_into().unwrap_or_else(|_| unreachable!());
        let mut ext_se = ExtSerializer::new(tag, &mut self.writer);
        value.serialize(&mut ext_se)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        match len {
            Some(len) => {
                rmp::encode::write_array_len(&mut self.writer, len as u32)?;
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
                rmp::encode::write_map_len(&mut self.writer, len as u32)?;
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
    default: Option<NonNull<pyo3::ffi::PyObject>>,
    opts: Opt,
) -> Result<NonNull<pyo3::ffi::PyObject>, String> {
    let mut buf = BytesWriter::default();
    let obj = PyObject::new(ptr, opts, 0, 0, default);
    let mut ser = MessagePackSerializer::new(&mut buf);
    let res = obj.serialize(&mut ser);
    match res {
        Ok(_) => Ok(buf.finish()),
        Err(err) => {
            ffi!(Py_DECREF(buf.finish().as_ptr()));
            Err(err.to_string())
        }
    }
}

#[derive(Copy, Clone)]
pub enum ObType {
    Str,
    Bytes,
    Int,
    Bool,
    None,
    Float,
    List,
    Dict,
    Datetime,
    Date,
    Time,
    Tuple,
    Uuid,
    Dataclass,
    NumpyScalar,
    NumpyArray,
    Pydantic,
    Enum,
    StrSubclass,
    Ext,
    Unknown,
}

pub fn pyobject_to_obtype(obj: *mut pyo3::ffi::PyObject, opts: Opt) -> ObType {
    let ob_type = ob_type!(obj);
    if is_type!(ob_type, STR_TYPE) {
        ObType::Str
    } else if is_type!(ob_type, BYTES_TYPE) {
        ObType::Bytes
    } else if is_type!(ob_type, INT_TYPE)
        && (opts & PASSTHROUGH_BIG_INT == 0
            || ffi!(_PyLong_NumBits(obj)) <= {
                if pylong_is_positive(obj) {
                    64
                } else {
                    63
                }
            })
    {
        ObType::Int
    } else if is_type!(ob_type, BOOL_TYPE) {
        ObType::Bool
    } else if is_type!(ob_type, NONE_TYPE) {
        ObType::None
    } else if is_type!(ob_type, FLOAT_TYPE) {
        ObType::Float
    } else if is_type!(ob_type, LIST_TYPE) {
        ObType::List
    } else if is_type!(ob_type, DICT_TYPE) {
        ObType::Dict
    } else if is_type!(ob_type, DATETIME_TYPE) && opts & PASSTHROUGH_DATETIME == 0 {
        ObType::Datetime
    } else {
        pyobject_to_obtype_unlikely(obj, opts)
    }
}

macro_rules! is_subclass {
    ($ob_type:expr, $flag:ident) => {
        unsafe { (((*$ob_type).tp_flags & pyo3::ffi::$flag) != 0) }
    };
}

#[inline(never)]
pub fn pyobject_to_obtype_unlikely(obj: *mut pyo3::ffi::PyObject, opts: Opt) -> ObType {
    let ob_type = ob_type!(obj);
    if is_type!(ob_type, DATE_TYPE) && opts & PASSTHROUGH_DATETIME == 0 {
        ObType::Date
    } else if is_type!(ob_type, TIME_TYPE) && opts & PASSTHROUGH_DATETIME == 0 {
        ObType::Time
    } else if is_type!(ob_type, TUPLE_TYPE) && opts & PASSTHROUGH_TUPLE == 0 {
        ObType::Tuple
    } else if is_type!(ob_type, UUID_TYPE) {
        ObType::Uuid
    } else if is_type!(ob_type!(ob_type), ENUM_TYPE) {
        ObType::Enum
    } else if opts & PASSTHROUGH_SUBCLASS == 0 && is_subclass!(ob_type, Py_TPFLAGS_UNICODE_SUBCLASS)
    {
        ObType::StrSubclass
    } else if opts & PASSTHROUGH_SUBCLASS == 0
        && is_subclass!(ob_type, Py_TPFLAGS_LONG_SUBCLASS)
        && (opts & PASSTHROUGH_BIG_INT == 0
            || ffi!(_PyLong_NumBits(obj)) <= {
                if pylong_is_positive(obj) {
                    64
                } else {
                    63
                }
            })
    {
        ObType::Int
    } else if opts & PASSTHROUGH_SUBCLASS == 0 && is_subclass!(ob_type, Py_TPFLAGS_LIST_SUBCLASS) {
        ObType::List
    } else if opts & PASSTHROUGH_SUBCLASS == 0 && is_subclass!(ob_type, Py_TPFLAGS_DICT_SUBCLASS) {
        ObType::Dict
    } else if opts & PASSTHROUGH_DATACLASS == 0 && pydict_contains!(ob_type, DATACLASS_FIELDS_STR) {
        ObType::Dataclass
    } else if opts & SERIALIZE_NUMPY != 0 && is_numpy_scalar(ob_type) {
        ObType::NumpyScalar
    } else if opts & SERIALIZE_NUMPY != 0 && is_numpy_array(ob_type) {
        ObType::NumpyArray
    } else if opts & SERIALIZE_PYDANTIC != 0
        && (pydict_contains!(ob_type, PYDANTIC_FIELDS_STR)
            || pydict_contains!(ob_type, PYDANTIC2_VALIDATOR_STR))
    {
        ObType::Pydantic
    } else if is_type!(ob_type, EXT_TYPE) {
        ObType::Ext
    } else {
        ObType::Unknown
    }
}

pub struct PyObject {
    ptr: *mut pyo3::ffi::PyObject,
    obtype: ObType,
    opts: Opt,
    default_calls: u8,
    recursion: u8,
    default: Option<NonNull<pyo3::ffi::PyObject>>,
}

impl PyObject {
    pub fn new(
        ptr: *mut pyo3::ffi::PyObject,
        opts: Opt,
        default_calls: u8,
        recursion: u8,
        default: Option<NonNull<pyo3::ffi::PyObject>>,
    ) -> Self {
        PyObject {
            ptr: ptr,
            obtype: pyobject_to_obtype(ptr, opts),
            opts: opts,
            default_calls: default_calls,
            recursion: recursion,
            default: default,
        }
    }
}

impl Serialize for PyObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.obtype {
            ObType::Str => Str::new(self.ptr).serialize(serializer),
            ObType::Bytes => Bytes::new(self.ptr).serialize(serializer),
            ObType::StrSubclass => StrSubclass::new(self.ptr).serialize(serializer),
            ObType::Int => Int::new(self.ptr).serialize(serializer),
            ObType::None => serializer.serialize_unit(),
            ObType::Float => serializer.serialize_f64(ffi!(PyFloat_AS_DOUBLE(self.ptr))),
            ObType::Bool => serializer.serialize_bool(unsafe { self.ptr == TRUE }),
            ObType::Datetime => match DateTime::new(self.ptr, self.opts) {
                Ok(val) => val.serialize(serializer),
                Err(err) => err!(err),
            },
            ObType::Date => Date::new(self.ptr).serialize(serializer),
            ObType::Time => match Time::new(self.ptr, self.opts) {
                Ok(val) => val.serialize(serializer),
                Err(err) => err!(err),
            },
            ObType::Uuid => UUID::new(self.ptr).serialize(serializer),
            ObType::Dict => {
                if unlikely!(self.recursion == RECURSION_LIMIT) {
                    err!(RECURSION_LIMIT_REACHED)
                }
                Dict::new(
                    self.ptr,
                    self.opts,
                    self.default_calls,
                    self.recursion,
                    self.default,
                )
                .serialize(serializer)
            }
            ObType::List => {
                if unlikely!(self.recursion == RECURSION_LIMIT) {
                    err!(RECURSION_LIMIT_REACHED)
                }
                List::new(
                    self.ptr,
                    self.opts,
                    self.default_calls,
                    self.recursion,
                    self.default,
                )
                .serialize(serializer)
            }
            ObType::Tuple => {
                if unlikely!(self.recursion == RECURSION_LIMIT) {
                    err!(RECURSION_LIMIT_REACHED)
                }
                Tuple::new(
                    self.ptr,
                    self.opts,
                    self.default_calls,
                    self.recursion,
                    self.default,
                )
                .serialize(serializer)
            }
            ObType::Dataclass => {
                if unlikely!(self.recursion == RECURSION_LIMIT) {
                    err!(RECURSION_LIMIT_REACHED)
                }
                Dataclass::new(
                    self.ptr,
                    self.opts,
                    self.default_calls,
                    self.recursion,
                    self.default,
                )
                .serialize(serializer)
            }
            ObType::Pydantic => {
                if unlikely!(self.recursion == RECURSION_LIMIT) {
                    err!(RECURSION_LIMIT_REACHED)
                }
                match AttributeDict::new(
                    self.ptr,
                    self.opts,
                    self.default_calls,
                    self.recursion,
                    self.default,
                ) {
                    Ok(val) => val.serialize(serializer),
                    Err(AttributeDictError::DictMissing) => err!(PYDANTIC_MUST_HAVE_DICT),
                }
            }
            ObType::Enum => {
                let value = ffi!(PyObject_GetAttr(self.ptr, VALUE_STR));
                ffi!(Py_DECREF(value));
                PyObject::new(
                    value,
                    self.opts,
                    self.default_calls,
                    self.recursion,
                    self.default,
                )
                .serialize(serializer)
            }
            ObType::NumpyArray => match NumpyArray::new(self.ptr, self.opts) {
                Ok(val) => val.serialize(serializer),
                Err(PyArrayError::Malformed) => err!("numpy array is malformed"),
                Err(PyArrayError::NotContiguous) | Err(PyArrayError::UnsupportedDataType) => {
                    if self.default.is_none() {
                        err!("numpy array is not C contiguous; use ndarray.tolist() in default")
                    } else {
                        Default::new(
                            self.ptr,
                            self.opts,
                            self.default_calls,
                            self.recursion,
                            self.default,
                        )
                        .serialize(serializer)
                    }
                }
            },
            ObType::NumpyScalar => NumpyScalar::new(self.ptr, self.opts).serialize(serializer),
            ObType::Ext => Ext::new(self.ptr).serialize(serializer),
            ObType::Unknown => Default::new(
                self.ptr,
                self.opts,
                self.default_calls,
                self.recursion,
                self.default,
            )
            .serialize(serializer),
        }
    }
}
