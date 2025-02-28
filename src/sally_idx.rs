//! `sally.idx` file holds the save index that lists save files.

use {
    byteorder::{LE, WriteBytesExt},
    std::{
        fs::OpenOptions,
        io::{Seek, SeekFrom},
        path::Path,
    },
};

pub struct IndexEntry {
    pub location: u16,
    pub entrance: u16,
    pub play_time: i32,
}

const ENTRY_LEN: u64 = 64;

impl IndexEntry {
    pub fn write_to_index(&self, path: &Path, slot: u8) -> std::io::Result<()> {
        let mut f = OpenOptions::new().write(true).open(path)?;
        f.seek(SeekFrom::Start(slot as u64 * ENTRY_LEN))?;
        f.write_u8(1)?;
        f.seek(SeekFrom::Current(7))?;
        f.write_u16::<LE>(self.location)?;
        f.write_u16::<LE>(self.entrance)?;
        f.seek(SeekFrom::Current(4))?;
        f.write_i32::<LE>(self.play_time)?;
        Ok(())
    }
}
