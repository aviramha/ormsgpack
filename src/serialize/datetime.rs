// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::opt::*;
use crate::serialize::datetimelike::{DateLike, DateTimeLike, Offset, TimeLike};
use crate::typeref::*;
use serde::ser::{Serialize, Serializer};

#[repr(transparent)]
pub struct Date {
    ptr: *mut pyo3::ffi::PyObject,
}

impl Date {
    pub fn new(ptr: *mut pyo3::ffi::PyObject) -> Self {
        Date { ptr: ptr }
    }
}

impl DateLike for Date {
    fn year(&self) -> i32 {
        ffi!(PyDateTime_GET_YEAR(self.ptr)) as i32
    }

    fn month(&self) -> i32 {
        ffi!(PyDateTime_GET_MONTH(self.ptr)) as i32
    }

    fn day(&self) -> i32 {
        ffi!(PyDateTime_GET_DAY(self.ptr)) as i32
    }
}

impl Serialize for Date {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut cursor = std::io::Cursor::new([0u8; 32]);
        DateLike::write_buf(self, &mut cursor).unwrap();
        let len = cursor.position() as usize;
        let value = unsafe { std::str::from_utf8_unchecked(&cursor.get_ref()[0..len]) };
        serializer.serialize_str(value)
    }
}

pub enum TimeError {
    HasTimezone,
}

impl std::fmt::Display for TimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HasTimezone => write!(f, "datetime.time must not have tzinfo set"),
        }
    }
}

pub struct Time {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
}

impl Time {
    pub fn new(ptr: *mut pyo3::ffi::PyObject, opts: Opt) -> Result<Self, TimeError> {
        if unsafe { (*(ptr as *mut pyo3::ffi::PyDateTime_Time)).hastzinfo != 0 } {
            return Err(TimeError::HasTimezone);
        }
        Ok(Time {
            ptr: ptr,
            opts: opts,
        })
    }
}

impl TimeLike for Time {
    fn hour(&self) -> i32 {
        ffi!(PyDateTime_TIME_GET_HOUR(self.ptr)) as i32
    }

    fn minute(&self) -> i32 {
        ffi!(PyDateTime_TIME_GET_MINUTE(self.ptr)) as i32
    }

    fn second(&self) -> i32 {
        ffi!(PyDateTime_TIME_GET_SECOND(self.ptr)) as i32
    }

    fn microsecond(&self) -> i32 {
        ffi!(PyDateTime_TIME_GET_MICROSECOND(self.ptr)) as i32
    }
}

impl Serialize for Time {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut cursor = std::io::Cursor::new([0u8; 32]);
        TimeLike::write_buf(self, &mut cursor, self.opts).unwrap();
        let len = cursor.position() as usize;
        let value = unsafe { std::str::from_utf8_unchecked(&cursor.get_ref()[0..len]) };
        serializer.serialize_str(value)
    }
}

pub enum DateTimeError {
    LibraryUnsupported,
}

impl std::fmt::Display for DateTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LibraryUnsupported => write!(f, "datetime's timezone library is not supported: use datetime.timezone.utc, pendulum, pytz, or dateutil"),
        }
    }
}

unsafe fn utcoffset(ptr: *mut pyo3::ffi::PyObject) -> Result<Offset, DateTimeError> {
    if (*(ptr as *mut pyo3::ffi::PyDateTime_DateTime)).hastzinfo != 1 {
        return Ok(Offset::default());
    }

    let tzinfo = pyo3::ffi::PyDateTime_DATE_GET_TZINFO(ptr);
    let py_offset: *mut pyo3::ffi::PyObject;
    if pyo3::ffi::PyObject_HasAttr(tzinfo, CONVERT_METHOD_STR) == 1 {
        // pendulum
        py_offset = pyo3::ffi::PyObject_CallMethodNoArgs(ptr, UTCOFFSET_METHOD_STR);
    } else if pyo3::ffi::PyObject_HasAttr(tzinfo, NORMALIZE_METHOD_STR) == 1 {
        // pytz
        let normalized = pyo3::ffi::PyObject_CallMethodOneArg(tzinfo, NORMALIZE_METHOD_STR, ptr);
        py_offset = pyo3::ffi::PyObject_CallMethodNoArgs(normalized, UTCOFFSET_METHOD_STR);
        pyo3::ffi::Py_DECREF(normalized);
    } else if pyo3::ffi::PyObject_HasAttr(tzinfo, DST_STR) == 1 {
        // dateutil/arrow, datetime.timezone.utc
        py_offset = pyo3::ffi::PyObject_CallMethodOneArg(tzinfo, UTCOFFSET_METHOD_STR, ptr);
    } else {
        return Err(DateTimeError::LibraryUnsupported);
    }
    let offset = Offset {
        day: pyo3::ffi::PyDateTime_DELTA_GET_DAYS(py_offset),
        second: pyo3::ffi::PyDateTime_DELTA_GET_SECONDS(py_offset),
    };
    pyo3::ffi::Py_DECREF(py_offset);
    Ok(offset)
}

pub struct DateTime {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    offset: Offset,
}

impl DateTime {
    pub fn new(ptr: *mut pyo3::ffi::PyObject, opts: Opt) -> Result<Self, DateTimeError> {
        let offset = unsafe { utcoffset(ptr)? };
        Ok(DateTime {
            ptr: ptr,
            opts: opts,
            offset: offset,
        })
    }
}

impl DateLike for DateTime {
    fn year(&self) -> i32 {
        ffi!(PyDateTime_GET_YEAR(self.ptr)) as i32
    }

    fn month(&self) -> i32 {
        ffi!(PyDateTime_GET_MONTH(self.ptr)) as i32
    }

    fn day(&self) -> i32 {
        ffi!(PyDateTime_GET_DAY(self.ptr)) as i32
    }
}

impl TimeLike for DateTime {
    fn hour(&self) -> i32 {
        ffi!(PyDateTime_DATE_GET_HOUR(self.ptr)) as i32
    }

    fn minute(&self) -> i32 {
        ffi!(PyDateTime_DATE_GET_MINUTE(self.ptr)) as i32
    }

    fn second(&self) -> i32 {
        ffi!(PyDateTime_DATE_GET_SECOND(self.ptr)) as i32
    }

    fn microsecond(&self) -> i32 {
        ffi!(PyDateTime_DATE_GET_MICROSECOND(self.ptr)) as i32
    }
}

impl DateTimeLike for DateTime {
    fn has_tz(&self) -> bool {
        unsafe { (*(self.ptr as *mut pyo3::ffi::PyDateTime_DateTime)).hastzinfo == 1 }
    }

    fn offset(&self) -> Offset {
        self.offset
    }
}

impl Serialize for DateTime {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut cursor = std::io::Cursor::new([0u8; 32]);
        DateTimeLike::write_buf(self, &mut cursor, self.opts).unwrap();
        let len = cursor.position() as usize;
        let value = unsafe { std::str::from_utf8_unchecked(&cursor.get_ref()[0..len]) };
        serializer.serialize_str(value)
    }
}
