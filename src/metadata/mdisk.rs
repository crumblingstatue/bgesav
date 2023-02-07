pub struct Entry {
    pub bit_idx: u8,
    pub name: &'static str,
}

pub const TABLE: [Entry; 14] = [
    Entry {
        name: "Game Save",
        bit_idx: 0,
    },
    Entry {
        name: "Mr. De Castellac",
        bit_idx: 3,
    },
    Entry {
        name: "The pearl and the currents",
        bit_idx: 2,
    },
    Entry {
        name: "For Jade",
        bit_idx: 4,
    },
    Entry {
        name: "Surveillance Camera",
        bit_idx: 11,
    },
    Entry {
        name: "Hillyan Army Databank",
        bit_idx: 1,
    },
    Entry {
        name: "Beluga Checkup",
        bit_idx: 10,
    },
    Entry {
        name: "Animal Species",
        bit_idx: 5,
    },
    Entry {
        name: "Iris 511",
        bit_idx: 6,
    },
    Entry {
        name: "Iris 512",
        bit_idx: 7,
    },
    Entry {
        name: "Iris 513",
        bit_idx: 8,
    },
    Entry {
        name: "Iris 514",
        bit_idx: 9,
    },
    Entry {
        name: "Disk Game",
        bit_idx: 12,
    },
    Entry {
        name: "Pearl Game",
        bit_idx: 13,
    },
];
