use std::{
    fs::{self, File},
    io::Cursor,
    path::Path,
};

use binrw::{BinRead, BinWriterExt};
use ds_rom::rom::{raw, Rom, RomLoadOptions};

use crate::dqmj1_rom::{btl_enmy_prm::BtlEnmyPrm, string_tables::StringTables};

#[tauri::command]
pub fn unpack_rom(rom_filepath: &str, temp_directory: &str) {
    let raw_rom = raw::Rom::from_file(rom_filepath).unwrap();
    let rom = Rom::extract(&raw_rom).unwrap();
    rom.save(temp_directory, None).unwrap();
}

#[tauri::command]
pub fn pack_rom(rom_filepath: &str, temp_directory: &str) {
    println!("Writing patched ROM to: {rom_filepath:?}");
    println!("Reading from temp dir: {temp_directory:?}");

    let config_filepath = Path::new(temp_directory).join("config.yaml");

    let rom = Rom::load(config_filepath, RomLoadOptions::default()).unwrap();
    let raw_rom = rom.build(None).unwrap();
    raw_rom.save(rom_filepath).unwrap();
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
pub fn set_btl_enmy_prm(temp_directory: &str, btl_enmy_prm: BtlEnmyPrm) {
    let filepath = Path::new(temp_directory)
        .join("files")
        .join("BtlEnmyPrm.bin");
    println!("Writing BtlEnmyPrm to: {filepath:?}");

    let mut file = File::create(filepath).unwrap();
    file.write_le(&btl_enmy_prm).unwrap();
}

#[tauri::command]
pub fn get_string_tables(temp_directory: &str) -> StringTables {
    let filepath = Path::new(temp_directory).join("arm9").join("arm9.bin");
    println!("Reading string tables from ARM9 binary: {filepath:?}");
    let file_data = fs::read(filepath).unwrap();

    StringTables::from_arm9(&file_data)
}
