use std::io::Cursor;

use binrw::{BinRead, BinResult};
use serde::{Deserialize, Serialize};

use crate::dqmj1_rom::regions::Region;

pub mod jp;
pub mod na;

#[derive(Debug, Deserialize, Serialize)]
pub enum ItemTblWithRegion {
    Jp(jp::ItemTbl),
    Na(na::ItemTbl),
}

impl ItemTblWithRegion {
    pub fn read(file_data: &[u8], region: Region) -> BinResult<Self> {
        let mut reader = Cursor::new(file_data);
        match region {
            Region::NorthAmerica => na::ItemTbl::read(&mut reader).map(ItemTblWithRegion::Na),
            Region::Japan => jp::ItemTbl::read(&mut reader).map(ItemTblWithRegion::Jp),
            Region::Europe => panic!("Europe not yet supported for ItemTbl parsing"),
        }
    }
}
