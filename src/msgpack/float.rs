// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::io::WriteSlices;
use crate::msgpack::marker::Marker;

#[inline]
pub fn write_f32<W>(writer: &mut W, value: f32) -> Result<(), std::io::Error>
where
    W: WriteSlices,
{
    writer.write_slices([&[Marker::F32.into()], &value.to_be_bytes()])
}

#[inline]
pub fn write_f64<W>(writer: &mut W, value: f64) -> Result<(), std::io::Error>
where
    W: WriteSlices,
{
    writer.write_slices([&[Marker::F64.into()], &value.to_be_bytes()])
}
