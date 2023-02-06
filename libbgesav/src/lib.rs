use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::Path,
};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

pub struct Sav {
    pub mdisks: Mdisks,
}

impl Sav {
    pub fn load_from_file(path: &Path) -> io::Result<Self> {
        let mut f = File::open(path)?;
        let mdisks = Mdisks::read(&mut f)?;
        Ok(Self { mdisks })
    }
    pub fn save_to_file(&self, path: &Path) -> io::Result<()> {
        let mut f = OpenOptions::new().write(true).open(path)?;
        self.mdisks.write(&mut f)?;
        Ok(())
    }
}

trait DatumRepr: Sized {
    fn read<R: Read>(src: &mut R) -> io::Result<Self>;
    fn write<W: Write>(self, dst: &mut W) -> io::Result<()>;
}

impl DatumRepr for u16 {
    fn read<R: Read>(src: &mut R) -> io::Result<Self> {
        src.read_u16::<LE>()
    }
    fn write<W: Write>(self, dst: &mut W) -> io::Result<()> {
        dst.write_u16::<LE>(self)
    }
}

trait SaveDatum: Sized {
    const OFFSET: usize;
    type Repr: DatumRepr;

    fn from_repr(repr: Self::Repr) -> Self;
    fn to_repr(&self) -> Self::Repr;

    fn read<R: Read + Seek>(src: &mut R) -> io::Result<Self> {
        src.seek(SeekFrom::Start(Self::OFFSET as _))?;
        let repr = Self::Repr::read(src)?;
        Ok(Self::from_repr(repr))
    }

    fn write<W: Write + Seek>(&self, dst: &mut W) -> io::Result<()> {
        dst.seek(SeekFrom::Start(Self::OFFSET as _))?;
        let repr = self.to_repr();
        repr.write(dst)
    }
}

#[derive(Debug)]
pub struct Mdisks {
    pub disks: [bool; 16],
}

impl SaveDatum for Mdisks {
    const OFFSET: usize = 600;
    type Repr = u16;

    fn from_repr(repr: Self::Repr) -> Self {
        dbg!(repr);
        let mut arr = [false; 16];
        for (i, elem) in arr.iter_mut().enumerate() {
            if repr >> i & 1 != 0 {
                *elem = true;
            }
        }
        Self { disks: arr }
    }

    fn to_repr(&self) -> Self::Repr {
        let mut repr = 0;
        for (i, has_disk) in self.disks.iter().copied().enumerate() {
            if has_disk {
                repr |= 1 << i;
            }
        }
        repr
    }
}
