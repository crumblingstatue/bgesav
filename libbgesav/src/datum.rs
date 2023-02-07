mod map;
mod mdisks;
mod party_present;
mod repr;

use std::io::{self, Read, Seek, SeekFrom, Write};

use self::repr::DatumRepr;
pub use self::{
    map::{MapEntry, MapId},
    mdisks::Mdisks,
    party_present::PartyPresent,
};

pub trait SaveDatum: Sized {
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
