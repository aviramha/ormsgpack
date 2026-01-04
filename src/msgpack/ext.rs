// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::io::WriteSlices;
use crate::msgpack::marker::Marker;

pub fn write_ext<W>(writer: &mut W, value: &[u8], tag: i8) -> Result<(), std::io::Error>
where
    W: WriteSlices,
{
    let len = value.len();
    if len == 1 {
        writer.write_slices([&[Marker::FixExt1.into(), tag as u8], value])
    } else if len == 2 {
        writer.write_slices([&[Marker::FixExt2.into(), tag as u8], value])
    } else if len == 4 {
        writer.write_slices([&[Marker::FixExt4.into(), tag as u8], value])
    } else if len == 8 {
        writer.write_slices([&[Marker::FixExt8.into(), tag as u8], value])
    } else if len == 16 {
        writer.write_slices([&[Marker::FixExt16.into(), tag as u8], value])
    } else if len < 256 {
        writer.write_slices([&[Marker::Ext8.into(), len as u8, tag as u8], value])
    } else if len < 65536 {
        writer.write_slices([
            &[Marker::Ext16.into()],
            &(len as u16).to_be_bytes(),
            &[tag as u8],
            value,
        ])
    } else if len <= 4294967295 {
        writer.write_slices([
            &[Marker::Ext32.into()],
            &(len as u32).to_be_bytes(),
            &[tag as u8],
            value,
        ])
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
    }
}
