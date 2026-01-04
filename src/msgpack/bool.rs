// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::msgpack::marker::Marker;

#[inline]
pub fn write_bool<W>(writer: &mut W, value: bool) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    let marker = if value { Marker::True } else { Marker::False };
    writer.write_all(&[marker.into()])
}
