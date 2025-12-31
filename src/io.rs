// SPDX-License-Identifier: (Apache-2.0 OR MIT)

pub trait WriteSlices: std::io::Write {
    fn write_slices<const N: usize>(&mut self, bufs: [&[u8]; N]) -> Result<(), std::io::Error>;
}

impl<T> WriteSlices for &mut T
where
    T: WriteSlices,
{
    fn write_slices<const N: usize>(&mut self, bufs: [&[u8]; N]) -> Result<(), std::io::Error> {
        (**self).write_slices(bufs)
    }
}
