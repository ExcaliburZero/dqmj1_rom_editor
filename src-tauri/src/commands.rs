use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Cursor,
    path::{Path, PathBuf},
    time::Instant,
};

use binrw::{BinRead, BinWrite, BinWriterExt};
use ds_rom::rom::{raw, Rom, RomLoadOptions};
use glob::glob;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use serde::Serialize;
use tauri::Manager;

use crate::dqmj1_rom::{
    btl_enmy_prm::BtlEnmyPrm,
    events::{
        assembly::parser::{parse_dqmj1_asm, ParseLexErrors},
        binary::Evt,
        disassembly::{DisassembledEvt, Opcode},
    },
    regions::Region,
    skill_tbl::SkillTblWithRegion,
    string_tables::StringTables,
    strings::encoding::CharacterEncoding,
};

fn get_mod_files(directory: &Path) -> Vec<String> {
    let mut mod_files = vec![
        "files/BtlEnmyPrm.bin".to_string(),
        "files/SkillTbl.bin".to_string(),
    ];

    let files_directory = directory.join("files");
    let event_files: Vec<String> = glob(&(files_directory.to_str().unwrap().to_owned() + "/*.evt"))
        .unwrap()
        .map(|fp| "files/".to_string() + fp.unwrap().file_name().unwrap().to_str().unwrap())
        .collect();

    mod_files.extend_from_slice(&event_files);

    mod_files
}

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

fn get_region(directory: &Path) -> Region {
    let header_filepath = directory.join("header.yaml");
    let header_data: BTreeMap<String, String> =
        serde_norway::from_str(&fs::read_to_string(header_filepath).unwrap()).unwrap();

    Region::from_game_code(header_data.get("gamecode").unwrap()).unwrap()
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
pub fn get_skill_tbl(app: tauri::AppHandle) -> SkillTblWithRegion {
    let temp_directory = get_temp_directory(&app);
    let region = get_region(&temp_directory);

    let filepath = temp_directory.join("files").join("SkillTbl.bin");
    println!("Reading SkillTbl from: {filepath:?}");
    let file_data = fs::read(filepath).unwrap();

    SkillTblWithRegion::read(&file_data, region).unwrap()
}

#[tauri::command]
pub fn set_skill_tbl(app: tauri::AppHandle, skill_tbl: SkillTblWithRegion) {
    let temp_directory = get_temp_directory(&app);
    //let region = get_region(&temp_directory); // TODO: check region

    let filepath = temp_directory.join("files").join("SkillTbl.bin");
    println!("Writing SkillTbl to: {filepath:?}");

    let mut file = File::create(filepath).unwrap();
    match skill_tbl {
        SkillTblWithRegion::Na(skill_tbl) => file.write_le(&skill_tbl).unwrap(),
        SkillTblWithRegion::Jp(skill_tbl) => file.write_le(&skill_tbl).unwrap(),
    };
}

#[tauri::command]
pub fn get_string_tables(app: tauri::AppHandle) -> StringTables {
    let temp_directory = get_temp_directory(&app);
    let region = get_region(&temp_directory);

    let filepath = temp_directory.join("arm9").join("arm9.bin");
    println!("Reading string tables from ARM9 binary: {filepath:?}");
    let file_data = fs::read(filepath).unwrap();

    StringTables::from_arm9(&file_data, region)
}

fn get_event_files(temp_directory: &Path) -> Vec<PathBuf> {
    fs::read_dir(temp_directory.join("files"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .filter(|path| path.extension().is_some_and(|extension| extension == "evt"))
        .collect()
}

#[tauri::command]
pub fn get_event_files_list(app: tauri::AppHandle) -> Vec<String> {
    let temp_directory = get_temp_directory(&app);

    get_event_files(&temp_directory)
        .iter()
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect()
}

fn disassemble_evt_file(
    character_encoding: &CharacterEncoding,
    opcodes: &[Opcode],
    evt_filepath: &Path,
    output_filepath: &Path,
) {
    println!("{:?} -> {:?}", evt_filepath, output_filepath);

    let mut reader = Cursor::new(std::fs::read(evt_filepath).unwrap());
    if let Ok(evt) = Evt::read(&mut reader) {
        let disassembled = DisassembledEvt::from_evt(&evt, character_encoding, opcodes);

        let mut file = File::create(output_filepath).unwrap();
        disassembled.write_asm(&mut file).unwrap();
    } else {
        println!("Failed to parse evt file: {:?}", evt_filepath);
    }
}

#[tauri::command]
pub fn export_events(app: tauri::AppHandle, output_directory: String) {
    let output_directory = Path::new(&output_directory);
    let temp_directory = get_temp_directory(&app);

    let region = get_region(&temp_directory);
    let character_encoding = CharacterEncoding::get(region);

    let opcodes = Opcode::get();

    get_event_files(&temp_directory)
        .par_iter()
        .for_each(|evt_filepath: &PathBuf| {
            let output_filepath = output_directory.join(
                evt_filepath
                    .with_extension("evt.dqmj1_asm")
                    .file_name()
                    .unwrap(),
            );
            disassemble_evt_file(
                &character_encoding,
                &opcodes,
                evt_filepath,
                &output_filepath,
            );
        });
}

fn assemble_event_asm_file(
    character_encoding: &CharacterEncoding,
    opcodes: &[Opcode],
    asm_filepath: &Path,
    output_filepath: &Path,
) -> Result<(), ParseLexErrors> {
    println!("{:?} -> {:?}", asm_filepath, output_filepath);

    let contents = std::fs::read_to_string(asm_filepath).unwrap();
    let disassembled = parse_dqmj1_asm(&contents, opcodes)?;
    let evt = disassembled.to_evt(character_encoding);

    let mut file = File::create(output_filepath).unwrap();
    evt.write_le(&mut file).unwrap();

    Ok(())
}

#[derive(Serialize, Debug)]
pub struct FileError {
    file: String,
    error: String,
}

#[tauri::command]
pub fn import_events(app: tauri::AppHandle, filepaths: Vec<String>) -> Vec<FileError> {
    // TODO: maybe write to a different dir and move to proper dir if all successful?
    let temp_directory = get_temp_directory(&app);
    let output_directory = temp_directory.join("files");

    let region = get_region(&temp_directory);
    let character_encoding = CharacterEncoding::get(region);

    let opcodes = Opcode::get();

    let results: Vec<_> = filepaths
        .par_iter()
        .map(|asm_filepath: &String| {
            let asm_filepath = Path::new(asm_filepath);
            let base_name = asm_filepath
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .split_once(".")
                .unwrap()
                .0;

            let output_filepath = output_directory.join(format!("{}.evt", base_name));
            (
                asm_filepath,
                assemble_event_asm_file(
                    &character_encoding,
                    &opcodes,
                    asm_filepath,
                    &output_filepath,
                ),
            )
        })
        .collect();

    if results.iter().all(|r| r.1.is_ok()) {
        vec![]
    } else {
        let mut errors = vec![];
        for (filepath, result) in results.iter() {
            if let Err(file_errors) = result {
                for error in file_errors.iter() {
                    errors.push(FileError {
                        file: filepath.file_name().unwrap().display().to_string(), //.display().to_string(),
                        error: error.to_string(),
                    });
                }
            }
        }

        errors
    }
}

#[tauri::command]
pub fn get_mods(app: tauri::AppHandle) -> Vec<String> {
    get_mod_names(&app)
}

#[tauri::command]
pub fn save_mod(app: tauri::AppHandle, mod_name: &str) {
    assert!(mod_name != "tmp");

    let temp_directory = get_temp_directory(&app);
    let mod_directory = get_mod_directory(&app, mod_name);

    assert!(temp_directory != mod_directory);

    for file in get_mod_files(&temp_directory).iter() {
        let source = temp_directory.join(file);
        let destination = mod_directory.join(file);

        assert!(source.exists());
        if let Err(error) = fs::copy(&source, &destination) {
            println!("Failed to copy file: {:?} => {:?}", source, destination);
            println!("  {:?}", error);
        };
    }
}

#[tauri::command]
pub fn load_mod(app: tauri::AppHandle, mod_name: &str) {
    let temp_directory = get_temp_directory(&app);
    let mod_directory = get_mod_directory(&app, mod_name);

    for file in get_mod_files(&mod_directory).iter() {
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
