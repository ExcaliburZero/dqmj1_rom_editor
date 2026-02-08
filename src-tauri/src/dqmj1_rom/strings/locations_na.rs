use crate::dqmj1_rom::strings::locations::{StringTableLocations, TableLocation};

pub const NA_STRING_TABLE_LOCATIONS: StringTableLocations = StringTableLocations {
    species_names: TableLocation {
        start: 0x207785C,
        end: 0x207805C,
    },
};
