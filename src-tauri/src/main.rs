// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
use modules::config::user_settings::UserSettings;
use modules::tauri_commands::get_latest_videos;
fn main() {
    tauri::Builder::default()
    .setup(|app| {
        UserSettings::load(&app.config())?;
        Ok(())
    })
        .invoke_handler(tauri::generate_handler![get_latest_videos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
