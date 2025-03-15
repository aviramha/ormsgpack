// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::msgpack::marker::Marker;
use byteorder::WriteBytesExt;

#[inline]
pub fn write_nil<W>(writer: &mut W) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    writer.write_u8(Marker::Null.into())
}
