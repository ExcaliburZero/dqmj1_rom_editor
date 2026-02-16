use crate::dqmj1_rom::{regions::Region, strings::locations_na::NA_STRING_TABLE_LOCATIONS};

pub struct TableLocation {
    pub start: u32,
    pub end: u32,
}

pub struct StringTableLocations {
    pub species_names: TableLocation,
    pub item_names: TableLocation,
    pub skill_names: TableLocation,
    pub trait_names: TableLocation,
    pub skill_set_names: TableLocation,
}

impl StringTableLocations {
    pub fn get(region: Region) -> StringTableLocations {
        match region {
            Region::NorthAmerica => NA_STRING_TABLE_LOCATIONS,
            _ => panic!(),
        }
    }
}
