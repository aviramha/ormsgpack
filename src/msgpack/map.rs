// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::msgpack::marker::Marker;
use byteorder::{BigEndian, WriteBytesExt};

pub fn write_map_len<W>(writer: &mut W, len: usize) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    if len < 16 {
        writer.write_u8(Marker::FixMap(len as u8).into())
    } else if len < 65536 {
        writer.write_u8(Marker::Map16.into())?;
        writer.write_u16::<BigEndian>(len as u16)
    } else if len <= 4294967295 {
        writer.write_u8(Marker::Map32.into())?;
        writer.write_u32::<BigEndian>(len as u32)
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
    }
}
