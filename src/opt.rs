// SPDX-License-Identifier: (Apache-2.0 OR MIT)

pub type Opt = u16;

pub const NAIVE_UTC: Opt = 1;
pub const NON_STR_KEYS: Opt = 1 << 1;
pub const OMIT_MICROSECONDS: Opt = 1 << 2;
pub const PASSTHROUGH_BIG_INT: Opt = 1 << 3;
pub const PASSTHROUGH_DATACLASS: Opt = 1 << 4;
pub const PASSTHROUGH_DATETIME: Opt = 1 << 5;
pub const PASSTHROUGH_SUBCLASS: Opt = 1 << 6;
pub const PASSTHROUGH_TUPLE: Opt = 1 << 7;
pub const SERIALIZE_NUMPY: Opt = 1 << 8;
pub const SERIALIZE_PYDANTIC: Opt = 1 << 9;
pub const SORT_KEYS: Opt = 1 << 10;
pub const UTC_Z: Opt = 1 << 11;

pub const NOT_PASSTHROUGH: Opt = !(PASSTHROUGH_BIG_INT
    | PASSTHROUGH_DATACLASS
    | PASSTHROUGH_DATETIME
    | PASSTHROUGH_SUBCLASS
    | PASSTHROUGH_TUPLE);

pub const MAX_PACKB_OPT: i32 = (NAIVE_UTC
    | NON_STR_KEYS
    | OMIT_MICROSECONDS
    | PASSTHROUGH_BIG_INT
    | PASSTHROUGH_DATETIME
    | PASSTHROUGH_DATACLASS
    | PASSTHROUGH_SUBCLASS
    | PASSTHROUGH_TUPLE
    | SERIALIZE_NUMPY
    | SERIALIZE_PYDANTIC
    | SORT_KEYS
    | UTC_Z) as i32;

pub const MAX_UNPACKB_OPT: i32 = NON_STR_KEYS as i32;
