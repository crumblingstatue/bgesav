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
