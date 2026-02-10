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
