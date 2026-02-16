use std::io::Cursor;

use binrw::{BinRead, BinResult};
use serde::{Deserialize, Serialize};

use crate::dqmj1_rom::regions::Region;

pub mod jp;
pub mod na;

#[derive(Debug, Deserialize, Serialize)]
pub enum SkillTblWithRegion {
    Jp(jp::SkillTbl),
    Na(na::SkillTbl),
}

impl SkillTblWithRegion {
    pub fn read(file_data: &[u8], region: Region) -> BinResult<Self> {
        let mut reader = Cursor::new(file_data);
        match region {
            Region::NorthAmerica => na::SkillTbl::read(&mut reader).map(SkillTblWithRegion::Na),
            Region::Japan => jp::SkillTbl::read(&mut reader).map(SkillTblWithRegion::Jp),
            Region::Europe => panic!("Europe not yet supported for SkillTbl parsing"),
        }
    }
}
