use std::{
    fs::{self, File},
    io::Cursor,
    path::PathBuf,
    time::Instant,
};

use binrw::{BinRead, BinWriterExt};
use ds_rom::rom::{raw, Rom, RomLoadOptions};
use tauri::Manager;

use crate::dqmj1_rom::{
    btl_enmy_prm::BtlEnmyPrm, skill_tbl::SkillTbl, string_tables::StringTables,
};

const MOD_FILES: [&str; 1] = ["files/BtlEnmyPrm.bin"];

fn get_app_directory(app: &tauri::AppHandle) -> PathBuf {
    let app_directory = app.path().app_data_dir().unwrap();

    fs::create_dir_all(&app_directory).unwrap();

    app_directory
}

fn get_mod_directory(app: &tauri::AppHandle, mod_name: &str) -> PathBuf {
    let app_directory = get_app_directory(app);
    let mod_directory = app_directory.join("mods").join(mod_name);

    fs::create_dir_all(&mod_directory).unwrap();
    fs::create_dir_all(mod_directory.join("files")).unwrap();

    mod_directory
}

fn get_temp_directory(app: &tauri::AppHandle) -> PathBuf {
    get_mod_directory(app, "tmp")
}

fn get_mod_names(app: &tauri::AppHandle) -> Vec<String> {
    let app_directory = get_app_directory(app);
    let mods_directory = app_directory.join("mods");

    fs::create_dir_all(&mods_directory).unwrap();

    fs::read_dir(mods_directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .filter(|path| path.file_name().unwrap() != "tmp")
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect()
}

#[tauri::command]
pub fn unpack_rom(app: tauri::AppHandle, rom_filepath: &str) {
    let temp_directory = get_temp_directory(&app);

    let now = Instant::now();

    let raw_rom = raw::Rom::from_file(rom_filepath).unwrap();
    let rom = Rom::extract(&raw_rom).unwrap();
    rom.save(temp_directory, None).unwrap();

    let elapsed = now.elapsed();
    println!("Unpacked ROM in: {elapsed:?}");
}

#[tauri::command]
pub fn pack_rom(app: tauri::AppHandle, rom_filepath: &str) {
    let temp_directory = get_temp_directory(&app);

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
    let temp_directory = get_temp_directory(&app);

    let filepath = temp_directory.join("files").join("BtlEnmyPrm.bin");
    println!("Reading BtlEnmyPrm from: {filepath:?}");
    let file_data = fs::read(filepath).unwrap();

    BtlEnmyPrm::read(&mut Cursor::new(file_data)).unwrap()
}

#[tauri::command]
pub fn set_btl_enmy_prm(app: tauri::AppHandle, btl_enmy_prm: BtlEnmyPrm) {
    let temp_directory = get_temp_directory(&app);

    let filepath = temp_directory.join("files").join("BtlEnmyPrm.bin");
    println!("Writing BtlEnmyPrm to: {filepath:?}");

    let mut file = File::create(filepath).unwrap();
    file.write_le(&btl_enmy_prm).unwrap();
}

#[tauri::command]
pub fn get_skill_tbl(app: tauri::AppHandle) -> SkillTbl {
    let temp_directory = get_temp_directory(&app);

    let filepath = temp_directory.join("files").join("SkillTbl.bin");
    println!("Reading SkillTbl from: {filepath:?}");
    let file_data = fs::read(filepath).unwrap();

    SkillTbl::read(&mut Cursor::new(file_data)).unwrap()
}

#[tauri::command]
pub fn set_skill_tbl(app: tauri::AppHandle, skill_tbl: SkillTbl) {
    let temp_directory = get_temp_directory(&app);

    let filepath = temp_directory.join("files").join("SkillTbl.bin");
    println!("Writing SkillTbl to: {filepath:?}");

    let mut file = File::create(filepath).unwrap();
    file.write_le(&skill_tbl).unwrap();
}

#[tauri::command]
pub fn get_string_tables(app: tauri::AppHandle) -> StringTables {
    let temp_directory = get_temp_directory(&app);

    let filepath = temp_directory.join("arm9").join("arm9.bin");
    println!("Reading string tables from ARM9 binary: {filepath:?}");
    let file_data = fs::read(filepath).unwrap();

    StringTables::from_arm9(&file_data)
}

#[tauri::command]
pub fn get_mods(app: tauri::AppHandle) -> Vec<String> {
    get_mod_names(&app)
}

#[tauri::command]
pub fn save_mod(app: tauri::AppHandle, mod_name: &str) {
    let temp_directory = get_temp_directory(&app);
    let mod_directory = get_mod_directory(&app, mod_name);

    for file in MOD_FILES.iter() {
        let source = temp_directory.join(file);
        let destination = mod_directory.join(file);

        fs::copy(source, destination).unwrap();
    }
}

#[tauri::command]
pub fn load_mod(app: tauri::AppHandle, mod_name: &str) {
    let temp_directory = get_temp_directory(&app);
    let mod_directory = get_mod_directory(&app, mod_name);

    for file in MOD_FILES.iter() {
        let source = mod_directory.join(file);
        let destination = temp_directory.join(file);

        if source.exists() {
            fs::copy(source, destination).unwrap();
        }
    }
}

#[tauri::command]
pub fn create_mod(app: tauri::AppHandle, mod_name: &str) {
    let mod_directory = get_mod_directory(&app, mod_name);

    fs::remove_dir_all(mod_directory).unwrap();
    get_mod_directory(&app, mod_name);
}
