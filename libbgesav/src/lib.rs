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
    DoubleHCurrHealth, DoubleHFollowState, DoubleHMaxHealth, HovercraftCurrHealth,
    HovercraftMaxHealth, JadeCurrHealth, JadeMaxHealth, MapEntry, MapId, Mdisks, OffsetedSaveDatum,
    PartyPresent, PeyjCurrHealth, PeyjFollowState, PeyjMaxHealth, Units,
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
    pub units: Units,
    pub peyj_max_health: PeyjMaxHealth,
    pub double_h_max_health: DoubleHMaxHealth,
    pub jade_max_health: JadeMaxHealth,
    pub hovercraft_max_health: HovercraftMaxHealth,
    pub peyj_curr_health: PeyjCurrHealth,
    pub double_h_curr_health: DoubleHCurrHealth,
    pub jade_curr_health: JadeCurrHealth,
    pub hovercraft_curr_health: HovercraftCurrHealth,
}
