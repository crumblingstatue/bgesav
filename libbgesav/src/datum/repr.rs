use std::io::{self, Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

pub trait DatumRepr: Sized {
    fn read<R: Read>(src: &mut R) -> io::Result<Self>;
    fn write<W: Write>(self, dst: &mut W) -> io::Result<()>;
}

impl DatumRepr for u8 {
    fn read<R: Read>(src: &mut R) -> io::Result<Self> {
        src.read_u8()
    }

    fn write<W: Write>(self, dst: &mut W) -> io::Result<()> {
        dst.write_u8(self)
    }
}

impl DatumRepr for u16 {
    fn read<R: Read>(src: &mut R) -> io::Result<Self> {
        src.read_u16::<LE>()
    }
    fn write<W: Write>(self, dst: &mut W) -> io::Result<()> {
        dst.write_u16::<LE>(self)
    }
}

impl DatumRepr for i32 {
    fn read<R: Read>(src: &mut R) -> io::Result<Self> {
        src.read_i32::<LE>()
    }

    fn write<W: Write>(self, dst: &mut W) -> io::Result<()> {
        dst.write_i32::<LE>(self)
    }
}

impl DatumRepr for f32 {
    fn read<R: Read>(src: &mut R) -> io::Result<Self> {
        src.read_f32::<LE>()
    }

    fn write<W: Write>(self, dst: &mut W) -> io::Result<()> {
        dst.write_f32::<LE>(self)
    }
}

impl<const N: usize> DatumRepr for [i32; N] {
    fn read<R: Read>(src: &mut R) -> io::Result<Self> {
        let mut buf = [0; N];
        src.read_i32_into::<LE>(&mut buf)?;
        Ok(buf)
    }

    fn write<W: Write>(self, dst: &mut W) -> io::Result<()> {
        for item in self {
            dst.write_i32::<LE>(item)?;
        }
        Ok(())
    }
}

impl<T: DatumRepr, const N: usize> DatumRepr for [T; N] {
    default fn read<R: Read>(src: &mut R) -> io::Result<Self> {
        std::array::try_from_fn(|_| T::read(src))
    }

    default fn write<W: Write>(self, dst: &mut W) -> io::Result<()> {
        for item in self {
            item.write(dst)?;
        }
        Ok(())
    }
}
