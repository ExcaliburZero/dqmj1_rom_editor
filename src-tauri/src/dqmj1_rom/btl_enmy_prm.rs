use binrw::binrw;
use serde::Serialize;

#[binrw]
#[brw(little)]
#[derive(Debug, Serialize)]
pub struct EnemySkill {
    pub unknown_a: [u8; 2],
    pub skill_id: u16,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Serialize)]
pub struct ItemDrop {
    pub item_id: u16,
    pub chance_denominator_2_power: u16,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Serialize)]
pub struct BtlEnmyPrmEntry {
    pub species_id: u16,
    pub unknown_a: [u8; 8],
    pub skills: [EnemySkill; 6],
    pub item_drops: [ItemDrop; 2],
    pub gold: u16,
    pub unknown_b: [u8; 2],
    pub exp: u16,
    pub unknown_c: [u8; 2],
    pub level: u8,
    pub unknown_d: u8,
    pub unknown_e: u8,
    pub scout_chance: u8,
    pub max_hp: u16,
    pub max_mp: u16,
    pub attack: u16,
    pub defense: u16,
    pub agility: u16,
    pub wisdom: u16,
    pub unknown_f: [u8; 20],
    pub skill_set_ids: [u8; 3],
    pub unknown_g: u8,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Serialize)]
pub struct BtlEnmyPrm {
    pub magic: u32, // TODO: use magic= field in brw instead
    pub length: u32,

    #[br(count = length)]
    pub entries: Vec<BtlEnmyPrmEntry>,
}
