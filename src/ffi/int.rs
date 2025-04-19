// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use serde::ser::{Serialize, Serializer};

// https://tools.ietf.org/html/rfc7159#section-6
// "[-(2**53)+1, (2**53)-1]"

pub enum IntError {
    Overflow,
}

impl std::fmt::Display for IntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overflow => write!(f, "Integer exceeds 64-bit range"),
        }
    }
}

pub enum Int {
    Signed(i64),
    Unsigned(u64),
}

impl Serialize for Int {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Int::Signed(value) => serializer.serialize_i64(*value),
            Int::Unsigned(value) => serializer.serialize_u64(*value),
        }
    }
}
