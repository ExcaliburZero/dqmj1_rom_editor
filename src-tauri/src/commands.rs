use std::{fs, io::Cursor, path::Path};

use binrw::BinRead;
use ds_rom::rom::{raw, Rom};

use crate::dqmj1_rom::{btl_enmy_prm::BtlEnmyPrm, string_tables::StringTables};

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
    println!("Reading BtlEnmyPrm from: {filepath:?}");
    let file_data = fs::read(filepath).unwrap();

    BtlEnmyPrm::read(&mut Cursor::new(file_data)).unwrap()
}

#[tauri::command]
pub fn get_string_tables(temp_directory: &str) -> StringTables {
    let filepath = Path::new(temp_directory).join("arm9").join("arm9.bin");
    println!("Reading string tables from ARM9 binary: {filepath:?}");
    let file_data = fs::read(filepath).unwrap();

    StringTables::from_arm9(&file_data)
}
