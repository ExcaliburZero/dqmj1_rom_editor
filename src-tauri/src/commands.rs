use std::{fs, io::Cursor, path::Path};

use binrw::BinRead;
use ds_rom::rom::{raw, Rom};

use crate::dqmj1_rom::btl_enmy_prm::BtlEnmyPrm;

#[tauri::command]
pub fn unpack_rom(rom_filepath: &str, temp_directory: &str) {
    let raw_rom = raw::Rom::from_file(rom_filepath).unwrap();
    let rom = Rom::extract(&raw_rom).unwrap();
    rom.save(temp_directory, None).unwrap();
}

#[tauri::command]
pub fn get_btl_enmy_prm(temp_directory: &str) -> BtlEnmyPrm {
    let filepath = Path::new(temp_directory)
        .join("files")
        .join("BtlEnmyPrm.bin");
    println!("Reading BtlEnmyPrm from: {:?}", filepath);
    let file_data = fs::read(filepath).unwrap();

    BtlEnmyPrm::read(&mut Cursor::new(file_data)).unwrap()
}
