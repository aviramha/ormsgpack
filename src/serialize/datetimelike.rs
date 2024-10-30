// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::opt::*;
use chrono::{Datelike, Timelike};
use serde::ser::{Serialize, Serializer};

pub type DateTimeBuffer = smallvec::SmallVec<[u8; 32]>;

fn write_integer(buf: &mut DateTimeBuffer, value: i32, width: usize) {
    let mut itoa_buf = itoa::Buffer::new();
    let formatted = itoa_buf.format(value);
    for _ in 0..width - formatted.len() {
        buf.push(b'0');
    }
    buf.extend_from_slice(formatted.as_bytes());
}

pub trait DateLike {
    fn year(&self) -> i32;
    fn month(&self) -> i32;
    fn day(&self) -> i32;

    fn write_buf(&self, buf: &mut DateTimeBuffer) {
        write_integer(buf, self.year(), 4);
        buf.push(b'-');
        write_integer(buf, self.month(), 2);
        buf.push(b'-');
        write_integer(buf, self.day(), 2);
    }
}

pub trait TimeLike {
    fn hour(&self) -> i32;
    fn minute(&self) -> i32;
    fn second(&self) -> i32;
    fn microsecond(&self) -> i32;

    fn write_buf(&self, buf: &mut DateTimeBuffer, opts: Opt) {
        write_integer(buf, self.hour(), 2);
        buf.push(b':');
        write_integer(buf, self.minute(), 2);
        buf.push(b':');
        write_integer(buf, self.second(), 2);
        if opts & OMIT_MICROSECONDS == 0 {
            let microsecond = self.microsecond();
            if microsecond != 0 {
                buf.push(b'.');
                write_integer(buf, microsecond, 6);
            }
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Offset {
    pub day: i32,
    pub second: i32,
}

pub trait DateTimeLike: DateLike + TimeLike {
    fn has_tz(&self) -> bool;
    fn offset(&self) -> Offset;

    fn write_buf(&self, buf: &mut DateTimeBuffer, opts: Opt) {
        DateLike::write_buf(self, buf);
        buf.push(b'T');
        TimeLike::write_buf(self, buf, opts);
        if self.has_tz() || opts & NAIVE_UTC != 0 {
            let offset = self.offset();
            if offset.second == 0 {
                if opts & UTC_Z != 0 {
                    buf.push(b'Z');
                } else {
                    buf.extend_from_slice(b"+00:00");
                }
            } else {
                let offset_hour: i32;
                let mut offset_minute: i32;
                let mut offset_second: i32;
                if offset.day == -1 {
                    // datetime.timedelta(days=-1, seconds=68400) -> -05:00
                    buf.push(b'-');
                    offset_second = 86400 - offset.second;
                } else {
                    // datetime.timedelta(seconds=37800) -> +10:30
                    buf.push(b'+');
                    offset_second = offset.second;
                }
                (offset_minute, offset_second) = (offset_second / 60, offset_second % 60);
                (offset_hour, offset_minute) = (offset_minute / 60, offset_minute % 60);
                // https://tools.ietf.org/html/rfc3339#section-5.8
                // "exactly 19 minutes and 32.13 seconds ahead of UTC"
                // "closest representable UTC offset"
                //  "+20:00"
                if offset_second >= 30 {
                    offset_minute += 1;
                }
                write_integer(buf, offset_hour, 2);
                buf.push(b':');
                write_integer(buf, offset_minute, 2);
            }
        }
    }
}

pub struct NaiveDateTime {
    pub dt: chrono::NaiveDateTime,
    pub opts: Opt,
}

impl DateLike for NaiveDateTime {
    fn year(&self) -> i32 {
        self.dt.year()
    }

    fn month(&self) -> i32 {
        self.dt.month() as i32
    }

    fn day(&self) -> i32 {
        self.dt.day() as i32
    }
}

impl TimeLike for NaiveDateTime {
    fn hour(&self) -> i32 {
        self.dt.hour() as i32
    }

    fn minute(&self) -> i32 {
        self.dt.minute() as i32
    }

    fn second(&self) -> i32 {
        self.dt.second() as i32
    }

    fn microsecond(&self) -> i32 {
        (self.dt.nanosecond() / 1_000) as i32
    }
}

impl DateTimeLike for NaiveDateTime {
    fn has_tz(&self) -> bool {
        false
    }

    fn offset(&self) -> Offset {
        Offset::default()
    }
}

impl Serialize for NaiveDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf = DateTimeBuffer::new();
        DateTimeLike::write_buf(self, &mut buf, self.opts);
        serializer.serialize_str(str_from_slice!(buf.as_ptr(), buf.len()))
    }
}
