// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::msgpack::marker::Marker;

pub fn write_str<W>(writer: &mut W, value: &str) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    let len = value.len();
    if len < 32 {
        writer.write_all(&[Marker::FixStr(len as u8).into()])?;
    } else if len < 256 {
        writer.write_all(&[Marker::Str8.into(), len as u8])?;
    } else if len < 65536 {
        writer.write_all(&[Marker::Str16.into()])?;
        writer.write_all(&(len as u16).to_be_bytes())?;
    } else if len <= 4294967295 {
        writer.write_all(&[Marker::Str32.into()])?;
        writer.write_all(&(len as u32).to_be_bytes())?;
    } else {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
    }
    writer.write_all(value.as_bytes())
}
