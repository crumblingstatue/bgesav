mod datum;

use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

use bgesav_derive::SavExt;
use datum::{MapEntry, MapId, Mdisks, SaveDatum};

pub trait SavExt: Sized {
    fn load_from_file(path: &Path) -> io::Result<Self>;
    fn save_to_file(&self, path: &Path) -> io::Result<()>;
}

#[derive(SavExt)]
pub struct Sav {
    pub mdisks: Mdisks,
    pub current_map: MapId,
    pub map_entry: MapEntry,
}
