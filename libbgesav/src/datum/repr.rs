use {
    byteorder::{LE, ReadBytesExt, WriteBytesExt},
    std::io::{self, Read, Write},
};

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

// I wanted to use specialization here, but since I also want this to build on
// stable Rust, I changed it to a manual implementation.
impl DatumRepr for [[i32; 7]; 30] {
    fn read<R: Read>(src: &mut R) -> io::Result<Self> {
        let mut buf = [[0i32; 7]; 30];
        for arr in &mut buf {
            *arr = <[i32; 7] as DatumRepr>::read(src)?;
        }
        Ok(buf)
    }

    fn write<W: Write>(self, dst: &mut W) -> io::Result<()> {
        for arr in self {
            arr.write(dst)?;
        }
        Ok(())
    }
}
