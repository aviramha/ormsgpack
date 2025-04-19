// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::ffi::*;
use crate::opt::*;
use crate::serialize::datetimelike::{DateLike, DateTimeBuffer, DateTimeLike, Offset, TimeLike};
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
        let mut buf = DateTimeBuffer::new();
        self.write_buf(&mut buf);
        serializer.serialize_str(str_from_slice!(buf.as_ptr(), buf.len()))
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
        let tzinfo = ffi!(PyDateTime_TIME_GET_TZINFO(ptr));
        if !py_is!(tzinfo, NONE) {
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
        let mut buf = DateTimeBuffer::new();
        self.write_buf(&mut buf, self.opts);
        serializer.serialize_str(str_from_slice!(buf.as_ptr(), buf.len()))
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

unsafe fn utcoffset(ptr: *mut pyo3::ffi::PyObject) -> Result<Option<Offset>, DateTimeError> {
    let tzinfo = pyo3::ffi::PyDateTime_DATE_GET_TZINFO(ptr);
    if py_is!(tzinfo, NONE) {
        return Ok(None);
    }
    let py_offset: *mut pyo3::ffi::PyObject;
    if pyo3::ffi::PyObject_HasAttr(tzinfo, CONVERT_METHOD_STR) == 1 {
        // pendulum
        py_offset = pyobject_call_method_no_args(ptr, UTCOFFSET_METHOD_STR);
    } else if pyo3::ffi::PyObject_HasAttr(tzinfo, NORMALIZE_METHOD_STR) == 1 {
        // pytz
        let normalized = pyobject_call_method_one_arg(tzinfo, NORMALIZE_METHOD_STR, ptr);
        py_offset = pyobject_call_method_no_args(normalized, UTCOFFSET_METHOD_STR);
        pyo3::ffi::Py_DECREF(normalized);
    } else if pyo3::ffi::PyObject_HasAttr(tzinfo, DST_STR) == 1 {
        // dateutil/arrow, datetime.timezone.utc
        py_offset = pyobject_call_method_one_arg(tzinfo, UTCOFFSET_METHOD_STR, ptr);
    } else {
        return Err(DateTimeError::LibraryUnsupported);
    }
    let offset = Offset {
        day: pyo3::ffi::PyDateTime_DELTA_GET_DAYS(py_offset),
        second: pyo3::ffi::PyDateTime_DELTA_GET_SECONDS(py_offset),
    };
    pyo3::ffi::Py_DECREF(py_offset);
    Ok(Some(offset))
}

pub struct DateTime {
    ptr: *mut pyo3::ffi::PyObject,
    opts: Opt,
    offset: Option<Offset>,
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
    fn offset(&self) -> Option<Offset> {
        self.offset
    }
}

impl Serialize for DateTime {
    #[inline(never)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf = DateTimeBuffer::new();
        DateTimeLike::write_buf(self, &mut buf, self.opts);
        serializer.serialize_str(str_from_slice!(buf.as_ptr(), buf.len()))
    }
}
