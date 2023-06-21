// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use std::path::PathBuf;
use std::thread;
use std::time::SystemTime;
use modules::config::Configuration;
use modules::tauri_commands::clip_exists;
use modules::tauri_commands::create_clip;
use modules::tauri_commands::get_latest_videos;
use modules::tauri_commands::get_timeline_thumbnails;
use modules::tauri_commands::get_user_settings;
use modules::tauri_commands::install_dependencies;
use modules::tauri_commands::verify_dependencies;
use tauri::Manager;
use tauri::http::ResponseBuilder;

use std::io::Write;
use tauri::http::{
    header::{ACCEPT_RANGES, CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE, RANGE},
    status::StatusCode,
    MimeType,
};

use crate::modules::tauri_commands::get_output_formats;

fn main() {
    let _ = setup_logger();



    tauri::Builder::default()
        .setup(|app| {
            let config = Configuration::init(&app.config());

            let thumbnails_location = PathBuf::from(config.app_config.lock().unwrap().thumbnail_cache.clone()); 

            thread::spawn(move || {
                modules::cleaning::clean_thumbnails(thumbnails_location);
            });

            app.manage(config);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_latest_videos,
            verify_dependencies,
            install_dependencies,
            create_clip,
            clip_exists,
            get_user_settings,
            get_timeline_thumbnails,
            get_output_formats
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