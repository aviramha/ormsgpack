// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::msgpack::marker::Marker;
use byteorder::{BigEndian, WriteBytesExt};

pub fn write_u64<W>(writer: &mut W, value: u64) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    if value < 128 {
        writer.write_u8(Marker::FixPos(value as u8).into())
    } else if value < 256 {
        writer.write_u8(Marker::U8.into())?;
        writer.write_u8(value as u8)
    } else if value < 65536 {
        writer.write_u8(Marker::U16.into())?;
        writer.write_u16::<BigEndian>(value as u16)
    } else if value <= 4294967295 {
        writer.write_u8(Marker::U32.into())?;
        writer.write_u32::<BigEndian>(value as u32)
    } else {
        writer.write_u8(Marker::U64.into())?;
        writer.write_u64::<BigEndian>(value)
    }
}

pub fn write_i64<W>(writer: &mut W, value: i64) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    if value < 0 {
        if value >= -32 {
            writer.write_u8(Marker::FixNeg(value as i8).into())
        } else if value >= -128 {
            writer.write_u8(Marker::I8.into())?;
            writer.write_i8(value as i8)
        } else if value >= -32768 {
            writer.write_u8(Marker::I16.into())?;
            writer.write_i16::<BigEndian>(value as i16)
        } else if value >= -2147483648 {
            writer.write_u8(Marker::I32.into())?;
            writer.write_i32::<BigEndian>(value as i32)
        } else {
            writer.write_u8(Marker::I64.into())?;
            writer.write_i64::<BigEndian>(value)
        }
    } else {
        write_u64(writer, value as u64)
    }
}
