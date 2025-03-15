// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::msgpack::marker::Marker;
use byteorder::{BigEndian, WriteBytesExt};

#[inline]
pub fn write_f32<W>(writer: &mut W, value: f32) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    writer.write_u8(Marker::F32.into())?;
    writer.write_f32::<BigEndian>(value)
}

#[inline]
pub fn write_f64<W>(writer: &mut W, value: f64) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    writer.write_u8(Marker::F64.into())?;
    writer.write_f64::<BigEndian>(value)
}
