use crate::commands::{
    create_mod, get_btl_enmy_prm, get_mods, get_skill_tbl, get_string_tables, load_mod, pack_rom,
    save_mod, set_btl_enmy_prm, set_skill_tbl, unpack_rom,
};

pub mod commands;
pub mod dqmj1_rom;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            unpack_rom,
            pack_rom,
            save_mod,
            load_mod,
            create_mod,
            get_mods,
            get_btl_enmy_prm,
            set_btl_enmy_prm,
            get_skill_tbl,
            set_skill_tbl,
            get_string_tables
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
