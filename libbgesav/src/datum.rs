mod map;
mod mdisks;
mod party_follow_state;
mod party_present;
mod repr;

use std::{
    borrow::Borrow,
    io::{self, Read, Seek, SeekFrom, Write},
};

use self::repr::DatumRepr;
pub use self::{
    map::{MapEntry, MapId},
    mdisks::Mdisks,
    party_follow_state::{DoubleHFollowState, FollowState, PeyjFollowState},
    party_present::PartyPresent,
};

pub trait SaveDatum: Sized {
    type Repr: DatumRepr;

    fn from_repr(repr: Self::Repr) -> Self;
    fn to_repr(&self) -> Self::Repr;
}

pub trait OffsetedSaveDatum: Sized
where
    Self: From<Self::Datum>,
    Self: Borrow<Self::Datum>,
{
    const OFFSET: usize;
    type Datum: SaveDatum;
    fn read<R: Read + Seek>(src: &mut R) -> io::Result<Self::Datum> {
        src.seek(SeekFrom::Start(Self::OFFSET as _))?;
        let repr = <Self::Datum as SaveDatum>::Repr::read(src)?;
        Ok(Self::Datum::from_repr(repr))
    }

    fn write<W: Write + Seek>(dat: &Self::Datum, dst: &mut W) -> io::Result<()> {
        dst.seek(SeekFrom::Start(Self::OFFSET as _))?;
        let repr = dat.to_repr();
        repr.write(dst)
    }
}

pub struct Units(pub i32);

impl SaveDatum for Units {
    type Repr = i32;

    fn from_repr(repr: Self::Repr) -> Self {
        Self(repr)
    }

    fn to_repr(&self) -> Self::Repr {
        self.0
    }
}

impl OffsetedSaveDatum for Units {
    const OFFSET: usize = 14348;

    type Datum = Self;
}
