mod mdisks;
mod party_follow_state;
mod party_present;
mod repr;

pub use self::repr::DatumRepr;
pub use self::{mdisks::Mdisks, party_follow_state::FollowState, party_present::PartyPresent};

pub trait SaveDatum: Sized {
    type Repr: DatumRepr;

    fn from_repr(repr: Self::Repr) -> Self;
    fn to_repr(&self) -> Self::Repr;
}

impl<T: DatumRepr + Copy> SaveDatum for T {
    type Repr = Self;

    fn from_repr(repr: Self::Repr) -> Self {
        repr
    }

    fn to_repr(&self) -> Self::Repr {
        *self
    }
}

impl SaveDatum for bool {
    type Repr = u8;

    fn from_repr(repr: Self::Repr) -> Self {
        repr != 0
    }

    fn to_repr(&self) -> Self::Repr {
        if *self {
            1
        } else {
            0
        }
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
