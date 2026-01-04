// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::io::WriteSlices;
use crate::msgpack::marker::Marker;

pub fn write_str<W>(writer: &mut W, value: &str) -> Result<(), std::io::Error>
where
    W: WriteSlices,
{
    let len = value.len();
    if len < 32 {
        writer.write_slices([
            &[Marker::FixStr(value.len() as u8).into()],
            value.as_bytes(),
        ])
    } else if len < 256 {
        writer.write_slices([&[Marker::Str8.into(), len as u8], value.as_bytes()])
    } else if len < 65536 {
        writer.write_slices([
            &[Marker::Str16.into()],
            &(len as u16).to_be_bytes(),
            value.as_bytes(),
        ])
    } else if len <= 4294967295 {
        writer.write_slices([
            &[Marker::Str32.into()],
            &(len as u32).to_be_bytes(),
            value.as_bytes(),
        ])
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
    }
}
