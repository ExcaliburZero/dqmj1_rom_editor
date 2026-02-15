use crate::dqmj1_rom::strings::locations::{StringTableLocations, TableLocation};

pub const NA_STRING_TABLE_LOCATIONS: StringTableLocations = StringTableLocations {
    species_names: TableLocation {
        start: 0x207785C,
        end: 0x207805C,
    },
    item_names: TableLocation {
        start: 0x20767E4,
        end: 0x2076BE8,
    },
    skill_names: TableLocation {
        start: 0x2076BE8,
        end: 0x207705C,
    },
    trait_names: TableLocation {
        start: 0x20757E0,
        end: 0x2075BE0,
    },
    skill_set_names: TableLocation {
        start: 0x20763E0,
        end: 0x20767E4,
    },
};
