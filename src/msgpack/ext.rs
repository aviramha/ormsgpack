// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::msgpack::marker::Marker;

pub fn write_ext<W>(writer: &mut W, value: &[u8], tag: i8) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    let len = value.len();
    if len == 1 {
        writer.write_all(&[Marker::FixExt1.into()])?;
    } else if len == 2 {
        writer.write_all(&[Marker::FixExt2.into()])?;
    } else if len == 4 {
        writer.write_all(&[Marker::FixExt4.into()])?;
    } else if len == 8 {
        writer.write_all(&[Marker::FixExt8.into()])?;
    } else if len == 16 {
        writer.write_all(&[Marker::FixExt16.into()])?;
    } else if len < 256 {
        writer.write_all(&[Marker::Ext8.into(), len as u8])?;
    } else if len < 65536 {
        writer.write_all(&[Marker::Ext16.into()])?;
        writer.write_all(&(len as u16).to_be_bytes())?;
    } else if len <= 4294967295 {
        writer.write_all(&[Marker::Ext32.into()])?;
        writer.write_all(&(len as u32).to_be_bytes())?;
    } else {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
    }
    writer.write_all(&[tag as u8])?;
    writer.write_all(value)
}
