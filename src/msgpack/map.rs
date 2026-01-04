// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::io::WriteSlices;
use crate::msgpack::marker::Marker;

pub fn write_map_len<W>(writer: &mut W, len: usize) -> Result<(), std::io::Error>
where
    W: WriteSlices,
{
    if len < 16 {
        writer.write_all(&[Marker::FixMap(len as u8).into()])
    } else if len < 65536 {
        writer.write_slices([&[Marker::Map16.into()], &(len as u16).to_be_bytes()])
    } else if len <= 4294967295 {
        writer.write_slices([&[Marker::Map32.into()], &(len as u32).to_be_bytes()])
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
    }
}
