// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::msgpack::marker::Marker;

pub fn write_bin<W>(writer: &mut W, value: &[u8]) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    let len = value.len();
    if len < 256 {
        writer.write_all(&[Marker::Bin8.into(), len as u8])?;
    } else if len < 65536 {
        writer.write_all(&[Marker::Bin16.into()])?;
        writer.write_all(&(len as u16).to_be_bytes())?;
    } else if len <= 4294967295 {
        writer.write_all(&[Marker::Bin32.into()])?;
        writer.write_all(&(len as u32).to_be_bytes())?;
    } else {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
    }
    writer.write_all(value)
}
