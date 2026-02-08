use serde::Serialize;

use crate::dqmj1_rom::strings::{
    encoding_na::get_na_character_encoding, locations::TableLocation,
    locations_na::NA_STRING_TABLE_LOCATIONS,
};

const POINTER_SIZE_IN_BYTES: usize = 4;

#[derive(Serialize)]
pub struct StringTables {
    pub species_names: Vec<String>,
    pub item_names: Vec<String>,
}

impl StringTables {
    pub fn from_arm9(arm9: &[u8]) -> StringTables {
        let table_locations = NA_STRING_TABLE_LOCATIONS;

        StringTables {
            species_names: Self::read_table(arm9, &table_locations.species_names),
            item_names: Self::read_table(arm9, &table_locations.item_names),
        }
    }

    pub fn read_table(arm9: &[u8], table_location: &TableLocation) -> Vec<String> {
        let offset: u32 = 0x02000000; // arm9.bin
        let character_encoding = get_na_character_encoding();

        let start = (table_location.start - offset) as usize;
        let end = (table_location.end - offset) as usize;

        let mut string_pointers: Vec<u32> = Vec::new();
        let mut current = start;
        while current < end {
            let pointer_bytes: [u8; 4] = arm9[current..current + POINTER_SIZE_IN_BYTES]
                .try_into()
                .unwrap();
            string_pointers.push(u32::from_le_bytes(pointer_bytes));

            current += POINTER_SIZE_IN_BYTES;
        }

        let mut strings: Vec<String> = Vec::with_capacity(string_pointers.len());
        for string_pointer in string_pointers {
            let string_pointer = (string_pointer - offset) as usize;

            strings.push(character_encoding.read_string(&arm9[string_pointer..]));
        }

        strings
    }
}
