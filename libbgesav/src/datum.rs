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

pub trait OffsetedSaveDatum: Sized {
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

impl SaveDatum for f32 {
    type Repr = Self;

    fn from_repr(repr: Self::Repr) -> Self {
        repr
    }

    fn to_repr(&self) -> Self::Repr {
        *self
    }
}

pub struct PeyjMaxHealth(pub f32);
pub struct DoubleHMaxHealth(pub f32);
pub struct JadeMaxHealth(pub f32);
pub struct HovercraftMaxHealth(pub f32);
pub struct PeyjCurrHealth(pub f32);
pub struct DoubleHCurrHealth(pub f32);
pub struct JadeCurrHealth(pub f32);
pub struct HovercraftCurrHealth(pub f32);

macro_rules! impl_unit_datum {
    (datum: $datum:ty; $($for:ty: $off:literal;)+) => {
        $(
            impl From<$datum> for $for {
                fn from(value: $datum) -> Self {
                    Self(value)
                }
            }

            impl Borrow<$datum> for $for {
                fn borrow(&self) -> &$datum {
                    &self.0
                }
            }

            impl OffsetedSaveDatum for $for {
                const OFFSET: usize = $off;

                type Datum = f32;
            }
        )+
    };
}

impl_unit_datum! {
    datum: f32;
    PeyjMaxHealth: 852;
    DoubleHMaxHealth: 856;
    JadeMaxHealth: 884;
    HovercraftMaxHealth: 892;
    PeyjCurrHealth: 13324;
    DoubleHCurrHealth: 13328;
    JadeCurrHealth: 13356;
    HovercraftCurrHealth: 13364;
}
