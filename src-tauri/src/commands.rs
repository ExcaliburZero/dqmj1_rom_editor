use std::{
    fs::{self, File},
    io::Cursor,
    path::PathBuf,
    time::Instant,
};

use binrw::{BinRead, BinWriterExt};
use ds_rom::rom::{raw, Rom, RomLoadOptions};
use tauri::Manager;

use crate::dqmj1_rom::{btl_enmy_prm::BtlEnmyPrm, string_tables::StringTables};

fn get_mod_directory(app: tauri::AppHandle, temp_directory: &str) -> PathBuf {
    let app_directory = app.path().app_data_dir().unwrap();

    fs::create_dir_all(&app_directory).unwrap();

    app_directory.join("mods").join(temp_directory)
}

fn get_temp_directory(app: tauri::AppHandle) -> PathBuf {
    get_mod_directory(app, "tmp")
}

#[tauri::command]
pub fn unpack_rom(app: tauri::AppHandle, rom_filepath: &str) {
    let temp_directory = get_temp_directory(app);

    let now = Instant::now();

    let raw_rom = raw::Rom::from_file(rom_filepath).unwrap();
    let rom = Rom::extract(&raw_rom).unwrap();
    rom.save(temp_directory, None).unwrap();

    let elapsed = now.elapsed();
    println!("Unpacked ROM in: {elapsed:?}");
}

#[tauri::command]
pub fn pack_rom(app: tauri::AppHandle, rom_filepath: &str) {
    let temp_directory = get_temp_directory(app);

    println!("Writing patched ROM to: {rom_filepath:?}");
    println!("Reading from temp dir: {temp_directory:?}");

    let config_filepath = temp_directory.join("config.yaml");

    let now = Instant::now();

    let rom = Rom::load(config_filepath, RomLoadOptions::default()).unwrap();
    let raw_rom = rom.build(None).unwrap();
    raw_rom.save(rom_filepath).unwrap();

    let elapsed = now.elapsed();
    println!("Packed ROM in: {elapsed:?}");
}

#[tauri::command]
pub fn get_btl_enmy_prm(app: tauri::AppHandle) -> BtlEnmyPrm {
    let temp_directory = get_temp_directory(app);

    let filepath = temp_directory.join("files").join("BtlEnmyPrm.bin");
    println!("Reading BtlEnmyPrm from: {filepath:?}");
    let file_data = fs::read(filepath).unwrap();

    BtlEnmyPrm::read(&mut Cursor::new(file_data)).unwrap()
}

#[tauri::command]
pub fn set_btl_enmy_prm(app: tauri::AppHandle, btl_enmy_prm: BtlEnmyPrm) {
    let temp_directory = get_temp_directory(app);

    let filepath = temp_directory.join("files").join("BtlEnmyPrm.bin");
    println!("Writing BtlEnmyPrm to: {filepath:?}");

    let mut file = File::create(filepath).unwrap();
    file.write_le(&btl_enmy_prm).unwrap();
}

#[tauri::command]
pub fn get_string_tables(app: tauri::AppHandle) -> StringTables {
    let temp_directory = get_temp_directory(app);

    let filepath = temp_directory.join("arm9").join("arm9.bin");
    println!("Reading string tables from ARM9 binary: {filepath:?}");
    let file_data = fs::read(filepath).unwrap();

    StringTables::from_arm9(&file_data)
}
