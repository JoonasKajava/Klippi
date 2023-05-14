// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
use modules::config::user_settings::UserSettings;
use modules::ffmpeg::get_version;
use modules::tauri_commands::get_latest_videos;
use modules::tauri_commands::verify_dependencies;
use modules::tauri_commands::install_dependencies;
fn main() {
    tauri::Builder::default()
    .setup(|app| {
        UserSettings::load(&app.config())?;
        println!("Version: {:?}", get_version("ffprobe"));
        println!("Version: {:?}", get_version("ffmpeg"));
        
        Ok(())
    })
        .invoke_handler(tauri::generate_handler![
            get_latest_videos,
            verify_dependencies,
            install_dependencies
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
