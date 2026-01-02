// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;

use modules::config::Configuration;
use modules::tauri_commands::clip_exists;
use modules::tauri_commands::create_clip;
use modules::tauri_commands::get_user_settings;
use modules::tauri_commands::install_dependencies;
use modules::tauri_commands::verify_dependencies;
use std::path::PathBuf;
use std::thread;
use std::time::SystemTime;
use tauri::Manager;

use crate::modules::tauri_commands::discover::discover_videos;
use crate::modules::tauri_commands::get_output_formats;
use crate::modules::tauri_commands::thumbnail::get_supported_video_extensions;
use crate::modules::tauri_commands::thumbnail::get_timeline_thumbnails;

fn main() {
    let _ = setup_logger();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let config = Configuration::init(&app.handle());

            let thumbnails_location =
                PathBuf::from(config.app_config.lock().unwrap().thumbnail_cache.clone());

            thread::spawn(move || {
                modules::cleaning::clean_thumbnails(thumbnails_location);
            });

            app.manage(config);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            discover_videos,
            verify_dependencies,
            install_dependencies,
            create_clip,
            clip_exists,
            get_user_settings,
            get_timeline_thumbnails,
            get_output_formats,
            get_supported_video_extensions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {} Ln {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
