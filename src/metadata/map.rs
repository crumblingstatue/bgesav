use std::collections::HashMap;

pub type MapInfo = HashMap<u8, MapDef>;

pub struct MapDef {
    pub name: &'static str,
    pub entrances: HashMap<u8, &'static str>,
}

macro_rules! maps {
    ($($idx:literal:$name:literal$([$($entr_idx:literal:$ent_name:literal)*])?)*) => {
        #[allow(unused_mut)]
        pub fn fill_map_info(maps: &mut MapInfo) {
         $(
            maps.insert($idx, MapDef {
               name: $name,
               entrances: {
                  let mut hm = HashMap::default();
                  $(
                     $(
                        hm.insert($entr_idx, $ent_name);
                     )*
                  )?
                  hm
               },
            });
         )*
        }
    };
}

maps! {
   31: "Hangar"
      [
         2: "Peyj's workshop"
         3: "Beluga landing"
      ]
   32: "Peyj's workshop"
   33: "Lightouse"
      [
         0: "Outside"
         1: "From hangar"
      ]
   34: "First boss map"
   35: "Destroyed lighthouse"
   46: "Factory Entrance"
   50: "Elevator room"
      [
         1: "Fence"
         2: "electrical closet"
         3: "tight spot"
         4: "laboratory"
         5: "elevator"
      ]
   51: "Electrical closet" [2: "elevator room"]
   52: "Laboratory" [2: "west wing"]
   53: "West wing" [2: "laboratory"]
   54: "Upper hall" [2: "grate" 3: "other grate" 4: "lasers" 5: "elevator"]
   55: "X-ray verification"
   56: "Closet"
   57: "Nutripills vat" [10: "in front of conveyor"]
   58: "Loading dock" [1: "normal entrance" 2: "closed bar corridor"]
   59: "Computer room"
   60: "Nutripills sewers (unused map)"
      [
         6: "Inside 1"
         9: "Inside 2"
      ]
   61: "Routing"
   62: "Control room"
   63: "Shipping room"
   72: "Mammago garage"
      [42: "Mdisk player"]
   80: "-"
   90: "Hyllis"
   [
      1: "Near factory"
      2: "Upside-down farmlands"
      3: "softlock run"
      4: "softlock run"
   ]
   91: "Space (crash)"
   92: "Selene (moon)"
   [
      1: "Near top"
      2: "Moon surface"
      7: "moon river"
      42: "mdisk player"
   ]
   93: "<crash>"
   94: "<crash>"
   100: "-"
   110: "Races 1 and 2"
   111: "<crash>"
   114: "Surveillance room"
   120: "Slaughterhouse road"
   121: "Slaughterhause quarter"
   122: "Exterior moats"
   123: "\"\""
   130: "Black isle"
   150: "Main canal"
   151: "Pedestrian district"
   152: "Akuda bar"
   [
      1: "main entrance"
      2: "hotel room"
      42: "bar mdisk"
   ]
   153: "Ming Tzu's shop" [2: "from pedestrian district"]
   158: "Iris Den"
   [
      2: "from bar"
      42: "iris mdisk"
   ]
   154: "Revolution scene"
   160: "Crash"
   161: "Crash"
   170: "Transmitter entrance" [42: "MDisk reader"]
   171: "Crash"
   180: "Cloister"
   [
      0: "Pey'j location"
      1:  "Alpha soldiers"
      2:  "Near great crypt"
      42: "mdisk reader"
   ]
   181: "The great crypt"
   220: "Looters cavern 1"
   221: "Looters cavern 2"
}
