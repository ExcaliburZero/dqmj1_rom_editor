use binrw::binrw;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[binrw]
#[brw(little)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    category: u8, // 0=?, 1=?, 2+ = equipment?
    unknown_ab: [u8; 3],
    restore_stat: u8,
    can_revive: u8,
    restore_min: u8,
    restore_max: u8,
    unknown_b: u8,
    weapon_type: u8, // 0 through 6
    unknown_c: [u8; 6],
    buy_value: u32,
    sell_value: u32,
    unknown_d: [u8; 2],
    attack_increase: u8,
    defense_increase: u8,
    agility_increase: u8,
    wisdom_increase: u8,
    max_hp_increase: u8,
    max_mp_increase: u8,
    #[serde(with = "BigArray")]
    unknown: [u8; 76],
}

#[binrw]
#[brw(little)]
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemTbl {
    pub magic: u32, // TODO: use magic= field in brw instead
    pub length: u32,

    #[br(count = length)]
    pub entries: Vec<Item>,
}
