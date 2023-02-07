use super::{OffsetedSaveDatum, SaveDatum};

#[derive(Debug, Clone, Copy)]
pub struct MapId(pub u8);

impl SaveDatum for MapId {
    type Repr = u8;

    fn from_repr(repr: Self::Repr) -> Self {
        Self(repr)
    }

    fn to_repr(&self) -> Self::Repr {
        self.0
    }
}

impl OffsetedSaveDatum for MapId {
    const OFFSET: usize = 612;
    type Datum = Self;
}

pub struct MapEntry(pub u8);

impl SaveDatum for MapEntry {
    type Repr = u8;

    fn from_repr(repr: Self::Repr) -> Self {
        Self(repr)
    }

    fn to_repr(&self) -> Self::Repr {
        self.0
    }
}

impl OffsetedSaveDatum for MapEntry {
    const OFFSET: usize = 11084;
    type Datum = Self;
}
