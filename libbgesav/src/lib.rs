mod datum;

use std::{
    borrow::Borrow,
    fs::{File, OpenOptions},
    io,
    path::Path,
};

use bgesav_derive::SavExt;
pub use datum::FollowState;
use datum::{
    DoubleHFollowState, MapEntry, MapId, Mdisks, OffsetedSaveDatum, PartyPresent, PeyjFollowState,
};

pub trait SavExt: Sized {
    fn load_from_file(path: &Path) -> io::Result<Self>;
    fn save_to_file(&self, path: &Path) -> io::Result<()>;
}

#[derive(SavExt)]
pub struct Sav {
    pub mdisks: Mdisks,
    pub current_map: MapId,
    pub map_entry: MapEntry,
    pub party: PartyPresent,
    pub peyj_follow_state: PeyjFollowState,
    pub double_h_follow_state: DoubleHFollowState,
}
