use crate::dqmj1_rom::strings::locations::{StringTableLocations, TableLocation};

pub const JP_STRING_TABLE_LOCATIONS: StringTableLocations = StringTableLocations {
    species_names: TableLocation {
        start: 0x0208E8E8,
        end: 0x0208F0E8,
    },
    item_names: TableLocation {
        start: 0x0208D8DC,
        end: 0x0208DCE0,
    },
    skill_names: TableLocation {
        start: 0x0208DCE0,
        end: 0x208E27C, // Not correct end location, but includes all the needed strings
    },
    trait_names: TableLocation {
        start: 0x0208C8D8,
        end: 0x0208CCD8,
    },
    skill_set_names: TableLocation {
        start: 0x0208D4D8,
        end: 0x0208D8DC,
    },
};
