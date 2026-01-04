// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::io::WriteSlices;
use crate::msgpack::marker::Marker;

pub fn write_bin<W>(writer: &mut W, value: &[u8]) -> Result<(), std::io::Error>
where
    W: WriteSlices,
{
    let len = value.len();
    if len < 256 {
        writer.write_slices([&[Marker::Bin8.into(), len as u8], value])
    } else if len < 65536 {
        writer.write_slices([&[Marker::Bin16.into()], &(len as u16).to_be_bytes(), value])
    } else if len <= 4294967295 {
        writer.write_slices([&[Marker::Bin32.into()], &(len as u32).to_be_bytes(), value])
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
    }
}
