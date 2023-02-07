mod datum;

use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

use datum::{Mdisks, SaveDatum};

pub struct Sav {
    pub mdisks: Mdisks,
}

impl Sav {
    pub fn load_from_file(path: &Path) -> io::Result<Self> {
        let mut f = File::open(path)?;
        let mdisks = Mdisks::read(&mut f)?;
        Ok(Self { mdisks })
    }
    pub fn save_to_file(&self, path: &Path) -> io::Result<()> {
        let mut f = OpenOptions::new().write(true).open(path)?;
        self.mdisks.write(&mut f)?;
        Ok(())
    }
}
