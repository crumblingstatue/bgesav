#![feature(min_specialization, array_try_from_fn)]

mod datum;

use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, Write},
    path::Path,
};

pub use datum::{DatumRepr, FollowState, Mdisks, PartyPresent, SaveDatum};

fn read_datum<T: SaveDatum, R: Read>(reader: &mut R) -> io::Result<T> {
    <T::Repr as DatumRepr>::read(reader).map(|val| T::from_repr(val))
}

fn write_datum<T: SaveDatum, W: Write>(writer: &mut W, value: &T) -> io::Result<()> {
    <T::Repr as DatumRepr>::write(value.to_repr(), writer)
}

macro_rules! sav_def {
    ($($name:ident $offset:literal $type:ty)*) => {
        pub struct Sav {
            $(
                pub $name: $type,
            )*
        }
        impl Sav {
            pub fn load_from_file(path: &Path) -> io::Result<Self> {
                let mut f = File::open(path)?;
                $(
                    f.seek(std::io::SeekFrom::Start($offset))?;
                    let $name = read_datum(&mut f)?;
                )*
                Ok(Self {
                    $(
                        $name,
                    )*
                })
            }
            pub fn save_to_file(&self, path: &Path) -> io::Result<()> {
                let mut f = OpenOptions::new().write(true).open(path)?;
                $(
                    f.seek(std::io::SeekFrom::Start($offset))?;
                    write_datum(&mut f, &self.$name)?;
                )*
                Ok(())
            }
        }
    };
}

pub type Inventory = [i32; 50];
pub type Password = [i32; 7];
pub type Passwords = [Password; 30];

sav_def! {
    // name                   offset type
       mdisks                 600    Mdisks
       disable_subtitles      604    bool
       current_map            612    u8
       peyj_max_health        852    f32
       double_h_max_health    856    f32
       jade_max_health        884    f32
       hovercraft_max_health  892    f32
       peyj_inventory         1000   Inventory
       double_h_inventory     1200   Inventory
       jade_inventory         2600   Inventory
       hovercraft_inventory   3000   Inventory
       map_entry              11084  u8
       passwords              11244  Passwords
       peyj_curr_health       13324  f32
       pearls                 13264  i32
       double_h_curr_health   13328  f32
       jade_curr_health       13356  f32
       hovercraft_curr_health 13364  f32
       peyj_follow_state      14300  FollowState
       double_h_follow_state  14304  FollowState
       units                  14348  i32
       party                  14543  PartyPresent
}
