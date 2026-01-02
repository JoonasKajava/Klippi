use anyhow::{Result};
use log::info;
use tauri::Manager;
use tauri::api::path::app_data_dir;
use std::{path::PathBuf};
use tauri::State;
use tauri::Window;
use ts_rs::TS;

use crate::modules::config::user_settings::UserSettings;
use crate::modules::ffmpeg::VersionResultError::NotInstalled;
use crate::modules::ffmpeg::VersionResultError::ParseError;
use crate::modules::ffmpeg::models::output_format::Limitation;

use super::config::Configuration;
use super::ffmpeg::ffmpeg::get_ffmpeg_location;
use super::ffmpeg::models::output_format::OutputFormat;
use super::{
    ffmpeg::{
        ffmpeg_factory::{
            create_clip_command,
        },
        get_version,
        installer::install_ffmpeg,
        models::clip_creation_options::ClipCreationOptions,
    },
    utils::filesystem_utils::PathBufExtensions,
};

pub mod discover;
pub mod thumbnail;

#[derive(Clone, serde::Serialize, TS)]
#[ts(export, export_to = "../src/lib/models/")]
pub struct VideoData {
    thumbnail: String,
    file: String,
    name: String,
}

#[tauri::command]
pub async fn clip_exists(file: PathBuf, settings: State<'_, Configuration>) -> Result<bool, ()> {
    let user_settings = settings.user_settings.lock().unwrap();
    Ok(PathBuf::from(user_settings.clip_location.clone())
        .join(file)
        .exists())
}

#[derive(Clone, serde::Serialize, TS)]
#[ts(export, export_to = "../src/lib/models/")]
pub enum TimelineThumbnailsResult {
    Generating(PathBuf),
    Found(PathBuf),
}

#[tauri::command]
pub async fn verify_dependencies(
    window: Window,
) -> Result<Vec<String>, &'static str> {
    info!("Verifying dependencies");
    let mut failed_dependencies: Vec<String> = Vec::new();

    let ffmpeg_location = get_ffmpeg_location(&window.app_handle().config());

    let ffmpeg_version = get_version("ffmpeg", &ffmpeg_location);
    let ffprobe_version = get_version("ffprobe", &ffmpeg_location);

    if let Err(e) = ffmpeg_version {
        match e {
            NotInstalled => failed_dependencies.push("ffmpeg".into()),
            ParseError => return Err("Failed to read ffmpeg version"),
        };
    }

    if let Err(e) = ffprobe_version {
        match e {
            NotInstalled => failed_dependencies.push("ffprobe".into()),
            ParseError => return Err("Failed to read ffprobe version"),
        };
    }
    Ok(failed_dependencies)
}
#[tauri::command]
pub async fn install_dependencies(
    window: Window
) -> Result<String, String> {
    let config = window.app_handle().config();
    let ffmpeg_location = app_data_dir(&config).expect("Unable to get app data dir");
    let result = install_ffmpeg(window, &ffmpeg_location).await;
    match result {
        Ok(_) => Ok("Successful".into()),
        Err(e) => {
            println!("{:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn create_clip(
    window: Window,
    options: ClipCreationOptions,
    settings: State<'_, Configuration>,
) -> Result<String, String> {
    let mut final_options = ClipCreationOptions {
        to: PathBuf::from(&settings.user_settings.lock().unwrap().clip_location).join(options.to),
        ..options
    };

    final_options.to.set_extension(final_options.format.to_string().to_lowercase());

    let command =
        create_clip_command(&final_options, &window.config()).map_err(|_| "Unable to get command".to_string())?;

    command
        .run(move |progress| {
            window
                .emit("ffmpeg_progress", progress)
                .expect("Could not emit event");
        })
        .await
        .map_err(|_| "Running failed!")?
        .wait()
        .map_err(|_| "Wait failed")?;

    final_options.to.reveal();

    Ok("Success".into())
}

#[tauri::command]
pub async fn get_user_settings(settings: State<'_, Configuration>) -> Result<UserSettings, ()> {
    return Ok(settings.user_settings.lock().unwrap().clone());
}

#[tauri::command]
pub fn get_output_formats() -> Vec<OutputFormat> {
    vec![
        OutputFormat {
            name: "mp4",
            extension: "mp4",
            preset: "ultrafast",
            limitations: vec![],
        },
        OutputFormat {
            name: "webm",
            extension: "webm",
            preset: "ultrafast",
            limitations: vec![],
        },
        OutputFormat {
            name: "gif",
            extension: "gif",
            preset: "ultrafast",
            limitations: vec![Limitation::NoBitrate, Limitation::NoAudio],
        },
        OutputFormat {
            name: "webp",
            extension: "webp",
            preset: "default",
            limitations: vec![Limitation::NoBitrate, Limitation::NoAudio],
        },
    ]
}
