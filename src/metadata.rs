pub struct MdiskEntry {
    pub bit_idx: u8,
    pub name: &'static str,
}

pub const MDISK_TABLE: [MdiskEntry; 14] = [
    MdiskEntry {
        name: "Game Save",
        bit_idx: 0,
    },
    MdiskEntry {
        name: "Mr. De Castellac",
        bit_idx: 3,
    },
    MdiskEntry {
        name: "The pearl and the currents",
        bit_idx: 2,
    },
    MdiskEntry {
        name: "For Jade",
        bit_idx: 4,
    },
    MdiskEntry {
        name: "Surveillance Camera",
        bit_idx: 11,
    },
    MdiskEntry {
        name: "Hillyan Army Databank",
        bit_idx: 1,
    },
    MdiskEntry {
        name: "Beluga Checkup",
        bit_idx: 10,
    },
    MdiskEntry {
        name: "Animal Species",
        bit_idx: 5,
    },
    MdiskEntry {
        name: "Iris 511",
        bit_idx: 6,
    },
    MdiskEntry {
        name: "Iris 512",
        bit_idx: 7,
    },
    MdiskEntry {
        name: "Iris 513",
        bit_idx: 8,
    },
    MdiskEntry {
        name: "Iris 514",
        bit_idx: 9,
    },
    MdiskEntry {
        name: "Disk Game",
        bit_idx: 12,
    },
    MdiskEntry {
        name: "Pearl Game",
        bit_idx: 13,
    },
];
