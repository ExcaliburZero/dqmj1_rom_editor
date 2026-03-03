use binrw::binrw;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[binrw]
#[brw(little)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Tokugi {
    pub unknown_a: [u8; 2],
    pub mp_cost: u16,
    pub unknown_b: [u8; 4], // TODO: separate byte 3 as "target" (single, multiple, ???, ???)
    pub enemy_range_lower_damage: u16,
    pub enemy_range_upper_damage: u16,
    pub ally_range_lower_damage: u16,
    pub ally_range_upper_damage: u16,
    pub min_wisdom: u16,
    pub max_wisdom: u16,
    pub variation: u16,
    pub max_damage: u16,
    pub unknown_c: [u8; 4],
    pub value_ptr: u32,
    pub unknown_d: [u8; 12],
}

#[binrw]
#[brw(little)]
#[derive(Debug, Deserialize, Serialize)]
pub struct TokugiDataTbl {
    #[serde(with = "BigArray")]
    pub entries: [Tokugi; 256],
}
