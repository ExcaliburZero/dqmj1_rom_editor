use std::{fs, path::Path};

use csv::Writer;
use serde::Serialize;

use crate::dqmj1_rom::{
    btl_enmy_prm::{BtlEnmyPrm, BtlEnmyPrmEntry},
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
