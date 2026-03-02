use std::{fs, path::Path};

use csv::Writer;
use serde::Serialize;

use crate::dqmj1_rom::{
    btl_enmy_prm::{BtlEnmyPrm, BtlEnmyPrmEntry},
    skill_tbl::SkillTblWithRegion,
    string_tables::StringTables,
    tokugi_data_tbl::{Tokugi, TokugiDataTbl},
};

pub struct AllData {
    pub btl_enmy_prm: BtlEnmyPrm,
    pub skill_tbl: SkillTblWithRegion,
    pub tokugi_data_tbl: TokugiDataTbl,
    pub string_tables: StringTables,
}

impl AllData {
    pub fn write_spreadsheets(&self, directory: &Path) {
        fs::create_dir_all(directory).unwrap();

        self.write_btl_enmy_prm(&directory.join("encounters.csv"));
        self.write_tokugi_data_tbl(&directory.join("skills.csv"));
    }

    pub fn write_btl_enmy_prm(&self, filepath: &Path) {
        let mut wtr = Writer::from_path(filepath).unwrap();

        for (i, row) in self.btl_enmy_prm.entries.iter().enumerate() {
            let encounter = Encounter::from_btl_enmy_prm(i, row, &self.string_tables);
            wtr.serialize(encounter).unwrap();
        }

        wtr.flush().unwrap();
    }

    pub fn write_tokugi_data_tbl(&self, filepath: &Path) {
        let mut wtr = Writer::from_path(filepath).unwrap();

        for (i, row) in self.tokugi_data_tbl.entries.iter().enumerate() {
            let encounter = Skill::from_tokugi_data_tbl(i, row, &self.string_tables);
            wtr.serialize(encounter).unwrap();
        }

        wtr.flush().unwrap();
    }
}

#[derive(Serialize)]
struct Encounter {
    pub species_id: u16,
    pub species_name: String,
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
        }
    }
}

#[derive(Serialize)]
struct Skill {
    pub skill_id: usize,
    pub skill_name: String,
    pub unknown_a: String,
    pub range_lower_damage_a: u16,
    pub range_upper_damage_a: u16,
    pub range_lower_damage_b: u16,
    pub range_upper_damage_b: u16,
    pub min_wisdom: u16,
    pub max_wisdom: u16,
    pub variation: u16,
    pub max_damage: u16,
    pub unknown_b: String,
    pub value_ptr: u32,
    pub unknown_c: String,
}

impl Skill {
    pub fn from_tokugi_data_tbl(
        tokugi_id: usize,
        tokugi_data_tbl: &Tokugi,
        string_tables: &StringTables,
    ) -> Skill {
        Skill {
            skill_id: tokugi_id,
            skill_name: string_tables.skill_names[tokugi_id].clone(),
            unknown_a: tokugi_data_tbl
                .unknown_a
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(","),
            range_lower_damage_a: tokugi_data_tbl.range_lower_damage_a,
            range_upper_damage_a: tokugi_data_tbl.range_upper_damage_a,
            range_lower_damage_b: tokugi_data_tbl.range_lower_damage_b,
            range_upper_damage_b: tokugi_data_tbl.range_upper_damage_b,
            min_wisdom: tokugi_data_tbl.min_wisdom,
            max_wisdom: tokugi_data_tbl.max_wisdom,
            variation: tokugi_data_tbl.variation,
            max_damage: tokugi_data_tbl.max_damage,
            unknown_b: tokugi_data_tbl
                .unknown_b
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(","),
            value_ptr: tokugi_data_tbl.value_ptr,
            unknown_c: tokugi_data_tbl
                .unknown_c
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(","),
        }
    }
}
