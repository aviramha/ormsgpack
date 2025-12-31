// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::msgpack::marker::Marker;

pub fn write_array_len<W>(writer: &mut W, len: usize) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    if len < 16 {
        writer.write_all(&[Marker::FixArray(len as u8).into()])
    } else if len < 65536 {
        writer.write_all(&[Marker::Array16.into()])?;
        writer.write_all(&(len as u16).to_be_bytes())
    } else if len <= 4294967295 {
        writer.write_all(&[Marker::Array32.into()])?;
        writer.write_all(&(len as u32).to_be_bytes())
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
    }
}
