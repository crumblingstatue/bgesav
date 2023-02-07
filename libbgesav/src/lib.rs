mod datum;

use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

use datum::{MapId, Mdisks, SaveDatum};

pub struct Sav {
    pub mdisks: Mdisks,
    pub current_map: MapId,
}

impl Sav {
    pub fn load_from_file(path: &Path) -> io::Result<Self> {
        let mut f = File::open(path)?;
        let mdisks = Mdisks::read(&mut f)?;
        let current_map = MapId::read(&mut f)?;
        Ok(Self {
            mdisks,
            current_map,
        })
    }
    pub fn save_to_file(&self, path: &Path) -> io::Result<()> {
        let mut f = OpenOptions::new().write(true).open(path)?;
        self.mdisks.write(&mut f)?;
        self.current_map.write(&mut f)?;
        Ok(())
    }
}
