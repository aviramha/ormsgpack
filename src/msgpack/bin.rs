// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::msgpack::marker::Marker;
use byteorder::{BigEndian, WriteBytesExt};

pub fn write_bin<W>(writer: &mut W, value: &[u8]) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    let len = value.len();
    if len < 256 {
        writer.write_u8(Marker::Bin8.into())?;
        writer.write_u8(len as u8)?;
    } else if len < 65536 {
        writer.write_u8(Marker::Bin16.into())?;
        writer.write_u16::<BigEndian>(len as u16)?;
    } else if len <= 4294967295 {
        writer.write_u8(Marker::Bin32.into())?;
        writer.write_u32::<BigEndian>(len as u32)?;
    } else {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
    }
    writer.write_all(value)
}
