use binrw::binrw;
use serde::{Deserialize, Serialize};

#[binrw]
#[brw(little)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SkillPointsRequirement {
    pub points_delta: u16,
    pub points_total: u16,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SkillEntry {
    pub skill_ids: [u16; 4],
    pub unknown: [u8; 4],
}

#[binrw]
#[brw(little)]
#[derive(Debug, Deserialize, Serialize)]
pub struct TraitEntry {
    pub trait_ids: [u8; 4],
}

#[binrw]
#[brw(little)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SkillTblEntry {
    pub can_upgrade: u8,
    pub category: u8,
    pub max_skill_points: u8,
    pub unknown_a: u8,
    pub skill_point_requirements: [SkillPointsRequirement; 10],
    pub skills: [SkillEntry; 10],
    pub traits: [TraitEntry; 10],
    pub skill_set_id: u16,
    pub unknown_b: [u8; 2],
    pub species_ids: [u16; 6],
    pub unknown_c: [u32; 5],
}

#[binrw]
#[brw(little)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SkillTbl {
    pub magic: u32, // TODO: use magic= field in brw instead
    pub length: u32,

    #[br(count = length)]
    pub entries: Vec<SkillTblEntry>,
}
