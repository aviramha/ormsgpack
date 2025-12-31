// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::msgpack::marker::Marker;

pub fn write_u64<W>(writer: &mut W, value: u64) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    if value < 128 {
        writer.write_all(&[Marker::FixPos(value as u8).into()])
    } else if value < 256 {
        writer.write_all(&[Marker::U8.into(), value as u8])
    } else if value < 65536 {
        writer.write_all(&[Marker::U16.into()])?;
        writer.write_all(&(value as u16).to_be_bytes())
    } else if value <= 4294967295 {
        writer.write_all(&[Marker::U32.into()])?;
        writer.write_all(&(value as u32).to_be_bytes())
    } else {
        writer.write_all(&[Marker::U64.into()])?;
        writer.write_all(&value.to_be_bytes())
    }
}

pub fn write_i64<W>(writer: &mut W, value: i64) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    if value < 0 {
        if value >= -32 {
            writer.write_all(&[Marker::FixNeg(value as i8).into()])
        } else if value >= -128 {
            writer.write_all(&[Marker::I8.into(), value as u8])
        } else if value >= -32768 {
            writer.write_all(&[Marker::I16.into()])?;
            writer.write_all(&(value as i16).to_be_bytes())
        } else if value >= -2147483648 {
            writer.write_all(&[Marker::I32.into()])?;
            writer.write_all(&(value as i32).to_be_bytes())
        } else {
            writer.write_all(&[Marker::I64.into()])?;
            writer.write_all(&value.to_be_bytes())
        }
    } else {
        write_u64(writer, value as u64)
    }
}
