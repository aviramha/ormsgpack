use crate::opt::*;
use crate::serialize::datetimelike::NaiveDateTime;
use crate::typeref::{ARRAY_STRUCT_STR, DESCR_STR, DTYPE_STR};
use chrono::{DateTime, NaiveDate};
use pyo3::ffi::*;
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::os::raw::{c_char, c_int, c_void};

macro_rules! slice {
    ($ptr:expr, $size:expr) => {
        unsafe { std::slice::from_raw_parts($ptr, $size) }
    };
}

#[repr(C)]
pub struct PyCapsule {
    pub ob_base: PyObject,
    pub pointer: *mut c_void,
    pub name: *const c_char,
    pub context: *mut c_void,
    pub destructor: *mut c_void, // should be typedef void (*PyCapsule_Destructor)(PyObject *);
}

// https://numpy.org/doc/1.26/reference/arrays.interface.html#object.__array_struct__

#[repr(C)]
pub struct PyArrayInterface {
    pub two: c_int,
    pub nd: c_int,
    pub typekind: c_char,
    pub itemsize: c_int,
    pub flags: c_int,
    pub shape: *mut Py_intptr_t,
    pub strides: *mut Py_intptr_t,
    pub data: *mut c_void,
    pub descr: *mut PyObject,
}

#[derive(Clone, Copy)]
enum ItemType {
    BOOL,
    DATETIME64(NumpyDatetimeUnit),
    F16,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
}

impl ItemType {
    fn find(array: *mut PyArrayInterface, ptr: *mut PyObject) -> Option<ItemType> {
        match unsafe { ((*array).typekind, (*array).itemsize) } {
            (098, 1) => Some(ItemType::BOOL),
            (077, 8) => {
                let unit = NumpyDatetimeUnit::from_pyobject(ptr);
                Some(ItemType::DATETIME64(unit))
            }
            (102, 2) => Some(ItemType::F16),
            (102, 4) => Some(ItemType::F32),
            (102, 8) => Some(ItemType::F64),
            (105, 1) => Some(ItemType::I8),
            (105, 2) => Some(ItemType::I16),
            (105, 4) => Some(ItemType::I32),
            (105, 8) => Some(ItemType::I64),
            (117, 1) => Some(ItemType::U8),
            (117, 2) => Some(ItemType::U16),
            (117, 4) => Some(ItemType::U32),
            (117, 8) => Some(ItemType::U64),
            _ => None,
        }
    }
}

pub enum PyArrayError {
    Malformed,
    NotContiguous,
    UnsupportedDataType,
}

struct NumpyArrayData {
    data: *const c_void,
    len: usize,
    kind: ItemType,
    opts: Opt,
}

impl Serialize for NumpyArrayData {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len)).unwrap();
        match self.kind {
            ItemType::BOOL => {
                let slice: &[u8] = slice!(self.data as *const u8, self.len);
                for &each in slice.iter() {
                    let value = each == 1;
                    seq.serialize_element(&value).unwrap();
                }
            }
            ItemType::DATETIME64(unit) => {
                let slice: &[i64] = slice!(self.data as *const i64, self.len);
                for &each in slice.iter() {
                    let value = unit
                        .datetime(each, self.opts)
                        .map_err(serde::ser::Error::custom)?;
                    seq.serialize_element(&value).unwrap();
                }
            }
            ItemType::F16 => {
                let slice: &[u16] = slice!(self.data as *const u16, self.len);
                for &each in slice.iter() {
                    let value = half::f16::from_bits(each).to_f32();
                    seq.serialize_element(&value).unwrap();
                }
            }
            ItemType::F32 => {
                let slice: &[f32] = slice!(self.data as *const f32, self.len);
                for &each in slice.iter() {
                    seq.serialize_element(&each).unwrap();
                }
            }
            ItemType::F64 => {
                let slice: &[f64] = slice!(self.data as *const f64, self.len);
                for &each in slice.iter() {
                    seq.serialize_element(&each).unwrap();
                }
            }
            ItemType::I8 => {
                let slice: &[i8] = slice!(self.data as *const i8, self.len);
                for &each in slice.iter() {
                    seq.serialize_element(&each).unwrap();
                }
            }
            ItemType::I16 => {
                let slice: &[i16] = slice!(self.data as *const i16, self.len);
                for &each in slice.iter() {
                    seq.serialize_element(&each).unwrap();
                }
            }
            ItemType::I32 => {
                let slice: &[i32] = slice!(self.data as *const i32, self.len);
                for &each in slice.iter() {
                    seq.serialize_element(&each).unwrap();
                }
            }
            ItemType::I64 => {
                let slice: &[i64] = slice!(self.data as *const i64, self.len);
                for &each in slice.iter() {
                    seq.serialize_element(&each).unwrap();
                }
            }
            ItemType::U8 => {
                let slice: &[u8] = slice!(self.data as *const u8, self.len);
                for &each in slice.iter() {
                    seq.serialize_element(&each).unwrap();
                }
            }
            ItemType::U16 => {
                let slice: &[u16] = slice!(self.data as *const u16, self.len);
                for &each in slice.iter() {
                    seq.serialize_element(&each).unwrap();
                }
            }
            ItemType::U32 => {
                let slice: &[u32] = slice!(self.data as *const u32, self.len);
                for &each in slice.iter() {
                    seq.serialize_element(&each).unwrap();
                }
            }
            ItemType::U64 => {
                let slice: &[u64] = slice!(self.data as *const u64, self.len);
                for &each in slice.iter() {
                    seq.serialize_element(&each).unwrap();
                }
            }
        }
        seq.end()
    }
}

enum NumpyArrayNode {
    Internal(Vec<NumpyArrayNode>),
    Leaf(NumpyArrayData),
}

impl Serialize for NumpyArrayNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Internal(children) => {
                let mut seq = serializer.serialize_seq(Some(children.len())).unwrap();
                for child in children {
                    seq.serialize_element(child).unwrap();
                }
                seq.end()
            }
            Self::Leaf(value) => value.serialize(serializer),
        }
    }
}

// >>> arr = numpy.array([[[1, 2], [3, 4]], [[5, 6], [7, 8]]], numpy.int32)
// >>> arr.ndim
// 3
// >>> arr.shape
// (2, 2, 2)
// >>> arr.strides
// (16, 8, 4)
pub struct NumpyArray {
    capsule: *mut PyObject,
    root: NumpyArrayNode,
}

impl NumpyArray {
    #[inline(never)]
    pub fn new(ptr: *mut PyObject, opts: Opt) -> Result<Self, PyArrayError> {
        let capsule = ffi!(PyObject_GetAttr(ptr, ARRAY_STRUCT_STR));
        let array = unsafe { (*(capsule as *mut PyCapsule)).pointer as *mut PyArrayInterface };
        if unsafe { (*array).two != 2 } {
            ffi!(Py_DECREF(capsule));
            Err(PyArrayError::Malformed)
        } else if unsafe { (*array).flags } & 0x1 != 0x1 {
            ffi!(Py_DECREF(capsule));
            Err(PyArrayError::NotContiguous)
        } else {
            let num_dimensions = unsafe { (*array).nd as usize };
            if num_dimensions == 0 {
                ffi!(Py_DECREF(capsule));
                return Err(PyArrayError::UnsupportedDataType);
            }
            match ItemType::find(array, ptr) {
                None => {
                    ffi!(Py_DECREF(capsule));
                    Err(PyArrayError::UnsupportedDataType)
                }
                Some(kind) => {
                    let root = if num_dimensions > 1 {
                        let mut position = Vec::with_capacity(num_dimensions);
                        NumpyArray::build(capsule as *mut PyCapsule, kind, opts, 0, &mut position)
                    } else {
                        let shape = slice!((*array).shape as *const isize, num_dimensions);
                        NumpyArrayNode::Leaf(NumpyArrayData {
                            data: unsafe { (*array).data },
                            len: shape[0] as usize,
                            kind: kind,
                            opts: opts,
                        })
                    };
                    Ok(NumpyArray {
                        capsule: capsule,
                        root: root,
                    })
                }
            }
        }
    }

    fn build(
        capsule: *mut PyCapsule,
        kind: ItemType,
        opts: Opt,
        depth: usize,
        position: &mut Vec<isize>,
    ) -> NumpyArrayNode {
        let array = unsafe { (*capsule).pointer as *mut PyArrayInterface };
        let num_dimensions = unsafe { (*array).nd as usize };
        let shape = slice!((*array).shape as *const isize, num_dimensions);
        let strides = slice!((*array).strides as *const isize, num_dimensions);
        let num_children = shape[depth];
        let mut children = Vec::with_capacity(num_children as usize);
        for i in 0..num_children {
            position.push(i);
            let child = if depth < num_dimensions - 2 {
                NumpyArray::build(capsule, kind, opts, depth + 1, position)
            } else {
                let offset = strides
                    .iter()
                    .zip(position.iter())
                    .map(|(a, b)| a * b)
                    .sum::<isize>();
                NumpyArrayNode::Leaf(NumpyArrayData {
                    data: unsafe { (*array).data.offset(offset) },
                    len: shape[num_dimensions - 1] as usize,
                    kind: kind,
                    opts: opts,
                })
            };
            position.pop();
            children.push(child);
        }
        NumpyArrayNode::Internal(children)
    }
}

impl Drop for NumpyArray {
    fn drop(&mut self) {
        ffi!(Py_DECREF(self.capsule));
    }
}

impl Serialize for NumpyArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.root.serialize(serializer)
    }
}

/// This mimicks the units supported by numpy's datetime64 type.
///
/// See
/// https://github.com/numpy/numpy/blob/v1.26.4/numpy/core/include/numpy/ndarraytypes.h#L244-L258
#[derive(Clone, Copy)]
enum NumpyDatetimeUnit {
    NaT,
    Years,
    Months,
    Weeks,
    Days,
    Hours,
    Minutes,
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
    Picoseconds,
    Femtoseconds,
    Attoseconds,
    Generic,
}

impl std::fmt::Display for NumpyDatetimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit = match self {
            Self::NaT => "NaT",
            Self::Years => "years",
            Self::Months => "months",
            Self::Weeks => "weeks",
            Self::Days => "days",
            Self::Hours => "hours",
            Self::Minutes => "minutes",
            Self::Seconds => "seconds",
            Self::Milliseconds => "milliseconds",
            Self::Microseconds => "microseconds",
            Self::Nanoseconds => "nanoseconds",
            Self::Picoseconds => "picoseconds",
            Self::Femtoseconds => "femtoseconds",
            Self::Attoseconds => "attoseconds",
            Self::Generic => "generic",
        };
        write!(f, "{}", unit)
    }
}

enum NumpyDateTimeError {
    UnsupportedUnit(NumpyDatetimeUnit),
    Unrepresentable { unit: NumpyDatetimeUnit, val: i64 },
}

impl std::fmt::Display for NumpyDateTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedUnit(unit) => write!(f, "unsupported numpy.datetime64 unit: {}", unit),
            Self::Unrepresentable { unit, val } => {
                write!(f, "unrepresentable numpy.datetime64: {} {}", val, unit)
            }
        }
    }
}

impl NumpyDatetimeUnit {
    /// Create a `NumpyDatetimeUnit` from a pointer to a Python object holding a
    /// numpy array.
    ///
    /// This function must only be called with pointers to numpy arrays.
    ///
    /// We need to look inside the `obj.dtype.descr` attribute of the Python
    /// object rather than using the `descr` field of the `__array_struct__`
    /// because that field isn't populated for datetime64 arrays; see
    /// https://github.com/numpy/numpy/issues/5350.
    fn from_pyobject(ptr: *mut PyObject) -> Self {
        let dtype = ffi!(PyObject_GetAttr(ptr, DTYPE_STR));
        let descr = ffi!(PyObject_GetAttr(dtype, DESCR_STR));
        let el0 = ffi!(PyList_GET_ITEM(descr, 0));
        let descr_str = ffi!(PyTuple_GET_ITEM(el0, 1));
        let uni = crate::unicode::unicode_to_str(descr_str).unwrap();
        if uni.len() < 5 {
            return Self::NaT;
        }
        // unit descriptions are found at
        // https://github.com/numpy/numpy/blob/v1.26.4/numpy/core/src/multiarray/datetime.c#L81-L98
        let ret = match &uni[4..uni.len() - 1] {
            "Y" => Self::Years,
            "M" => Self::Months,
            "W" => Self::Weeks,
            "D" => Self::Days,
            "h" => Self::Hours,
            "m" => Self::Minutes,
            "s" => Self::Seconds,
            "ms" => Self::Milliseconds,
            "us" => Self::Microseconds,
            "ns" => Self::Nanoseconds,
            "ps" => Self::Picoseconds,
            "fs" => Self::Femtoseconds,
            "as" => Self::Attoseconds,
            "generic" => Self::Generic,
            _ => unreachable!(),
        };
        ffi!(Py_DECREF(dtype));
        ffi!(Py_DECREF(descr));
        ret
    }

    /// Return a `NaiveDateTime` for a value in array with this unit.
    ///
    /// Returns an `Err(NumpyDateTimeError)` if the value is invalid for this unit.
    fn datetime(&self, val: i64, opts: Opt) -> Result<NaiveDateTime, NumpyDateTimeError> {
        match self {
            Self::Years => Ok(NaiveDate::from_ymd_opt(
                (val + 1970)
                    .try_into()
                    .map_err(|_| NumpyDateTimeError::Unrepresentable { unit: *self, val })?,
                1,
                1,
            )
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()),
            Self::Months => Ok(NaiveDate::from_ymd_opt(
                (val / 12 + 1970)
                    .try_into()
                    .map_err(|_| NumpyDateTimeError::Unrepresentable { unit: *self, val })?,
                (val % 12 + 1)
                    .try_into()
                    .map_err(|_| NumpyDateTimeError::Unrepresentable { unit: *self, val })?,
                1,
            )
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()),
            Self::Weeks => Ok(DateTime::from_timestamp(val * 7 * 24 * 60 * 60, 0)
                .unwrap()
                .naive_utc()),
            Self::Days => Ok(DateTime::from_timestamp(val * 24 * 60 * 60, 0)
                .unwrap()
                .naive_utc()),
            Self::Hours => Ok(DateTime::from_timestamp(val * 60 * 60, 0)
                .unwrap()
                .naive_utc()),
            Self::Minutes => Ok(DateTime::from_timestamp(val * 60, 0).unwrap().naive_utc()),
            Self::Seconds => Ok(DateTime::from_timestamp(val, 0).unwrap().naive_utc()),
            Self::Milliseconds => Ok(DateTime::from_timestamp(
                val / 1_000,
                (val % 1_000 * 1_000_000)
                    .try_into()
                    .map_err(|_| NumpyDateTimeError::Unrepresentable { unit: *self, val })?,
            )
            .unwrap()
            .naive_utc()),
            Self::Microseconds => Ok(DateTime::from_timestamp(
                val / 1_000_000,
                (val % 1_000_000 * 1_000)
                    .try_into()
                    .map_err(|_| NumpyDateTimeError::Unrepresentable { unit: *self, val })?,
            )
            .unwrap()
            .naive_utc()),
            Self::Nanoseconds => Ok(DateTime::from_timestamp(
                val / 1_000_000_000,
                (val % 1_000_000_000)
                    .try_into()
                    .map_err(|_| NumpyDateTimeError::Unrepresentable { unit: *self, val })?,
            )
            .unwrap()
            .naive_utc()),
            _ => Err(NumpyDateTimeError::UnsupportedUnit(*self)),
        }
        .map(|dt| NaiveDateTime { dt, opts })
    }
}

macro_rules! define_numpy_type {
    ($name:ident, $object_name:ident, $type:ty) => {
        #[repr(C)]
        struct $object_name {
            ob_base: PyObject,
            value: $type,
        }

        #[repr(transparent)]
        pub struct $name {
            ptr: *mut PyObject,
        }

        impl $name {
            pub fn new(ptr: *mut PyObject) -> Self {
                $name { ptr }
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let value = unsafe { (*(self.ptr as *mut $object_name)).value };
                value.serialize(serializer)
            }
        }
    };
}

define_numpy_type!(NumpyBool, NumpyBoolObject, bool);
define_numpy_type!(NumpyFloat32, NumpyFloat32Object, f32);
define_numpy_type!(NumpyFloat64, NumpyFloat64Object, f64);
define_numpy_type!(NumpyInt8, NumpyInt8Object, i8);
define_numpy_type!(NumpyInt16, NumpyInt16Object, i16);
define_numpy_type!(NumpyInt32, NumpyInt32Object, i32);
define_numpy_type!(NumpyInt64, NumpyInt64Object, i64);
define_numpy_type!(NumpyUint8, NumpyUint8Object, u8);
define_numpy_type!(NumpyUint16, NumpyUint16Object, u16);
define_numpy_type!(NumpyUint32, NumpyUint32Object, u32);
define_numpy_type!(NumpyUint64, NumpyUint64Object, u64);

#[repr(C)]
struct NumpyDatetime64Object {
    ob_base: PyObject,
    value: i64,
}

pub struct NumpyDatetime64 {
    ptr: *mut PyObject,
    opts: Opt,
}

impl NumpyDatetime64 {
    pub fn new(ptr: *mut PyObject, opts: Opt) -> Self {
        NumpyDatetime64 { ptr, opts }
    }
}

impl Serialize for NumpyDatetime64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let unit = NumpyDatetimeUnit::from_pyobject(self.ptr);
        let value = unsafe { (*(self.ptr as *mut NumpyDatetime64Object)).value };
        unit.datetime(value, self.opts)
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}

#[repr(C)]
struct NumpyFloat16Object {
    ob_base: PyObject,
    value: u16,
}

#[repr(transparent)]
pub struct NumpyFloat16 {
    ptr: *mut PyObject,
}

impl NumpyFloat16 {
    pub fn new(ptr: *mut PyObject) -> Self {
        NumpyFloat16 { ptr }
    }
}

impl Serialize for NumpyFloat16 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = unsafe { (*(self.ptr as *mut NumpyFloat16Object)).value };
        half::f16::from_bits(value).to_f32().serialize(serializer)
    }
}
