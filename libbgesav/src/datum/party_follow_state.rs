use std::borrow::Borrow;

use super::{OffsetedSaveDatum, SaveDatum};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FollowState {
    Follow,
    /// Pey'j is present in workshop (fixing mdisk reader), can't talk to him.
    Unknown1,
    /// Pey'j is missing from workshop
    Unknown2,
    /// Pey'j is present in workshop (fixing mdisk reader), can't talk to him.
    Unknown3,
    /// Pey'j is fixing mdisk reader in workshop, can talk to him
    Unknown4,
}

impl SaveDatum for FollowState {
    type Repr = u8;

    fn from_repr(repr: Self::Repr) -> Self {
        match repr {
            0 => FollowState::Follow,
            1 => FollowState::Unknown1,
            2 => FollowState::Unknown2,
            3 => FollowState::Unknown3,
            4 => FollowState::Unknown4,
            _ => panic!("Unknown follow state: {repr}"),
        }
    }

    fn to_repr(&self) -> Self::Repr {
        match *self {
            FollowState::Follow => 0,
            FollowState::Unknown1 => 1,
            FollowState::Unknown2 => 2,
            FollowState::Unknown3 => 3,
            FollowState::Unknown4 => 4,
        }
    }
}

pub struct PeyjFollowState(pub FollowState);

impl From<FollowState> for PeyjFollowState {
    fn from(value: FollowState) -> Self {
        Self(value)
    }
}

impl Borrow<FollowState> for PeyjFollowState {
    fn borrow(&self) -> &FollowState {
        &self.0
    }
}

impl OffsetedSaveDatum for PeyjFollowState {
    const OFFSET: usize = 14300;

    type Datum = FollowState;
}

pub struct DoubleHFollowState(pub FollowState);

impl From<FollowState> for DoubleHFollowState {
    fn from(value: FollowState) -> Self {
        Self(value)
    }
}

impl Borrow<FollowState> for DoubleHFollowState {
    fn borrow(&self) -> &FollowState {
        &self.0
    }
}

impl OffsetedSaveDatum for DoubleHFollowState {
    const OFFSET: usize = 14304;

    type Datum = FollowState;
}
