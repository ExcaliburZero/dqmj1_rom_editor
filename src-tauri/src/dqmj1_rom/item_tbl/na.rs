use binrw::binrw;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[binrw]
#[brw(little)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    category: u8, // 0=usable item, 1=key item, 2+ = equipment?
    unknown_ab: [u8; 3],
    effect: u8, // 1=restore hp, 2=restore mp, 3=revive, 4=cure poison, 5=cure paralysis, 6=cure all status effects, ...
    unknown_abb: u8,
    restore_min: u8,
    restore_max: u8,
    subcategory: u8,
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
