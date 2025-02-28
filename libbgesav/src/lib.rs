#![feature(min_specialization, array_try_from_fn)]
#![warn(clippy::pedantic)]
#![expect(clippy::missing_errors_doc)]

mod datum;

pub use datum::{DatumRepr, FollowState, Mdisks, PartyPresent, SaveDatum};
use {
    datum::hovercraft::{HovercraftCondition, HovercraftState},
    std::{
        fs::{File, OpenOptions},
        io::{self, Read, Seek, Write},
        path::Path,
    },
};

fn read_datum<T: SaveDatum, R: Read>(reader: &mut R) -> io::Result<T> {
    <T::Repr as DatumRepr>::read(reader).map(|val| T::from_repr(val))
}

fn write_datum<T: SaveDatum, W: Write>(writer: &mut W, value: &T) -> io::Result<()> {
    <T::Repr as DatumRepr>::write(value.to_repr(), writer)
}

macro_rules! sav_def {
    ($($name:ident $offset:literal $type:ty: $def:expr;)*) => {
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
                let mut f = OpenOptions::new().write(true).create(true).open(path)?;
                $(
                    f.seek(std::io::SeekFrom::Start($offset))?;
                    write_datum(&mut f, &self.$name)?;
                )*
                Ok(())
            }
        }
        impl Default for Sav {
            fn default() -> Self {
                Self {
                    $(
                        $name: $def,
                    )*
                }
            }
        }
    };
}

pub type Inventory = [i32; 49];
pub type Password = [i32; 7];
pub type Passwords = [Password; 30];

fn default_inv() -> Inventory {
    [0; 49]
}

sav_def! {
    // name                   offset type
       mdisks                 600    Mdisks: Mdisks::default();
       disable_subtitles      604    bool: true;
       current_map            612    u8: 33; // Lighthouse
       hangar_mystery_value   776    f32: -1.; // If not -1, things go crazy in Hangar map
       peyj_max_health        852    f32: 80.;
       double_h_max_health    856    f32: 80.;
       jade_max_health        884    f32: 80.;
       hovercraft_max_health  892    f32: 80.;
       inventory_offset       1000   i32: 50; // Needs to be 50 for things to function properly.
       peyj_inventory         1004   Inventory: default_inv();
       double_h_inventory     1204   Inventory: default_inv();
       jade_inventory         2604   Inventory: default_inv();
       hovercraft_inventory   3004   Inventory: default_inv();
       map_entry              11084  u8: 0;
       hovercraft_dock_x      11212  f32: 73.;
       hovercraft_dock_y      11216  f32: 1.46;
       passwords              11244  Passwords: Default::default();
       peyj_curr_health       13324  f32: 80.;
       pearls                 13264  i32: Default::default();
       hovercraft_dock_map    13312  u8: 31; // Hangar
       double_h_curr_health   13328  f32: 80.;
       jade_curr_health       13356  f32: 80.;
       hovercraft_curr_health 13364  f32: 80.;
       peyj_follow_state      14300  FollowState: FollowState::Follow;
       double_h_follow_state  14304  FollowState: FollowState::Follow;
       hovercraft_state       14340  HovercraftState: HovercraftState { value: 0 };
       units                  14348  i32: Default::default();
       party                  14543  PartyPresent: PartyPresent { peyj: false, double_h: false, alpha_soldier: false };
       hovercraft_condition   14696  HovercraftCondition: HovercraftCondition { bitflags: 0 };
}
