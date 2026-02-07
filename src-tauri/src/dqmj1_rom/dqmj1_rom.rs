use binrw::BinResult;
use ds_rom::rom::raw::Rom;
use ds_rom::rom::FileSystem;

use crate::dqmj1_rom::btl_enmy_prm::BtlEnmyPrm;

const NUM_DQMJ1_OVERLAYS: u16 = 3;

pub struct Dqmj1Rom<'a> {
    rom: Rom<'a>,
}

impl Dqmj1Rom<'_> {
    pub fn from_rom(rom: Rom) -> Dqmj1Rom {
        Dqmj1Rom { rom }
    }

    pub fn get_btl_enmy_prm(&self) -> BinResult<BtlEnmyPrm> {
        let fnt = self.rom.fnt().unwrap();
        let fat = self.rom.fat().unwrap();
        let root = FileSystem::parse(&fnt, fat, &self.rom).unwrap();

        let file_id = self.get_file_id("BtlEnmyPrm.bin");
        println!("{:?}", file_id);

        let file = root.file(file_id.unwrap());
        println!("{:?}", file.name());
        println!("{:?}", file.id());
        println!("{:?}", file.contents());

        panic!()

        //BtlEnmyPrm::read(&mut data)
    }

    pub fn get_file_id(&self, filename: &str) -> Option<u16> {
        let fnt = self.rom.fnt().unwrap();
        let fat = self.rom.fat().unwrap();
        let root = FileSystem::parse(&fnt, fat, &self.rom).unwrap();

        let max_id = root.max_file_id();
        for i in NUM_DQMJ1_OVERLAYS..=max_id {
            if FileSystem::is_file(i) {
                let file = root.file(i);
                if file.name() == filename {
                    return Some(i);
                }
            }
        }

        None
    }
}
