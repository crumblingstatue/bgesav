use super::SaveDatum;

#[derive(Debug, Clone, Copy)]
pub struct MapId(pub u8);

impl SaveDatum for MapId {
    const OFFSET: usize = 612;

    type Repr = u8;

    fn from_repr(repr: Self::Repr) -> Self {
        Self(repr)
    }

    fn to_repr(&self) -> Self::Repr {
        self.0
    }
}
