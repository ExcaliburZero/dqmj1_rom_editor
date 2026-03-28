use std::{fs, path::Path};

use csv::Writer;
use serde::Serialize;

use crate::dqmj1_rom::{
    btl_enmy_prm::{BtlEnmyPrm, BtlEnmyPrmEntry, EnemySkill, ItemDrop},
    skill_tbl::SkillTblWithRegion,
    string_tables::StringTables,
};

pub struct AllData {
    pub btl_enmy_prm: BtlEnmyPrm,
    pub skill_tbl: SkillTblWithRegion,
    pub string_tables: StringTables,
}

impl AllData {
    pub fn write_spreadsheets(&self, directory: &Path) {
        fs::create_dir_all(directory).unwrap();

        self.write_btl_enmy_prm(&directory.join("encounters.csv"));
    }

    pub fn write_btl_enmy_prm(&self, filepath: &Path) {
        let mut wtr = Writer::from_path(filepath).unwrap();

        for (i, row) in self.btl_enmy_prm.entries.iter().enumerate() {
            let encounter = Encounter::from_btl_enmy_prm(i, row, &self.string_tables);
            wtr.serialize(encounter).unwrap();
        }

        wtr.flush().unwrap();
    }
}

#[derive(Serialize)]
struct Encounter {
    pub species_id: u16,
    pub species_name: String,
    pub unknown_a: String,
    pub skill_1: String,
    pub skill_2: String,
    pub skill_3: String,
    pub skill_4: String,
    pub skill_5: String,
    pub skill_6: String,
    pub item_drop_1: String,
    pub item_drop_2: String,
    pub gold: u16,
    //pub unknown_b: String,
    pub exp: u16,
    //pub unknown_c: String,
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
    pub unknown_f: String,
    pub skill_set_1: String,
    pub skill_set_2: String,
    pub skill_set_3: String,
}

impl Encounter {
    pub fn from_btl_enmy_prm(
        _encounter_id: usize,
        btl_enmy_prm: &BtlEnmyPrmEntry,
        string_tables: &StringTables,
    ) -> Encounter {
        Encounter {
            species_id: btl_enmy_prm.species_id,
            species_name: string_tables.species_names[btl_enmy_prm.species_id as usize].clone(),
            unknown_a: bytes_to_string(&btl_enmy_prm.unknown_a),
            skill_1: format_encounter_skill(&btl_enmy_prm.skills[0], string_tables),
            skill_2: format_encounter_skill(&btl_enmy_prm.skills[1], string_tables),
            skill_3: format_encounter_skill(&btl_enmy_prm.skills[2], string_tables),
            skill_4: format_encounter_skill(&btl_enmy_prm.skills[3], string_tables),
            skill_5: format_encounter_skill(&btl_enmy_prm.skills[4], string_tables),
            skill_6: format_encounter_skill(&btl_enmy_prm.skills[5], string_tables),
            item_drop_1: format_encounter_item_drop(&btl_enmy_prm.item_drops[0], string_tables),
            item_drop_2: format_encounter_item_drop(&btl_enmy_prm.item_drops[1], string_tables),
            gold: btl_enmy_prm.gold,
            //unknown_b: bytes_to_string(&btl_enmy_prm.unknown_b),
            exp: btl_enmy_prm.exp,
            //unknown_c: bytes_to_string(&btl_enmy_prm.unknown_c),
            level: btl_enmy_prm.level,
            unknown_d: btl_enmy_prm.unknown_d,
            unknown_e: btl_enmy_prm.unknown_e,
            scout_chance: btl_enmy_prm.scout_chance,
            max_hp: btl_enmy_prm.max_hp,
            max_mp: btl_enmy_prm.max_mp,
            attack: btl_enmy_prm.attack,
            defense: btl_enmy_prm.defense,
            agility: btl_enmy_prm.agility,
            wisdom: btl_enmy_prm.wisdom,
            unknown_f: bytes_to_string(&btl_enmy_prm.unknown_f),
            skill_set_1: format_encounter_skill_set(btl_enmy_prm.skill_set_ids[0], string_tables),
            skill_set_2: format_encounter_skill_set(btl_enmy_prm.skill_set_ids[1], string_tables),
            skill_set_3: format_encounter_skill_set(btl_enmy_prm.skill_set_ids[2], string_tables),
        }
    }
}

fn bytes_to_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn format_encounter_skill(skill: &EnemySkill, string_tables: &StringTables) -> String {
    string_tables.skill_names[skill.skill_id as usize].to_string()
}

fn format_encounter_item_drop(item_drop: &ItemDrop, string_tables: &StringTables) -> String {
    if item_drop.item_id != 0 {
        format!(
            "{}  @ 1/{}",
            string_tables.item_names[item_drop.item_id as usize],
            2_u32.pow(item_drop.chance_denominator_2_power as u32)
        )
    } else {
        "".to_string()
    }
}

fn format_encounter_skill_set(skill_set_id: u8, string_tables: &StringTables) -> String {
    if skill_set_id != 0 {
        string_tables.skill_set_names[skill_set_id as usize].to_string()
    } else {
        "".to_string()
    }
}
