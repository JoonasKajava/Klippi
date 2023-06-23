use anyhow::{Context, Result};
use tauri::Manager;
use tauri::api::path::app_data_dir;
use std::{fs, path::PathBuf};
use tauri::State;
use tauri::Window;
use ts_rs::TS;

use crate::modules::cleaning::THUMBNAIL_EXTENSION;
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
            create_clip_command, create_thumbnail_command, create_timeline_thumbnails_command,
        },
        get_version,
        installer::install_ffmpeg,
        models::clip_creation_options::ClipCreationOptions,
    },
    file_processing::video_metadata::find_latest_videos,
    utils::filesystem_utils::PathBufExtensions,
};
use log::{error, info};

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
pub async fn get_timeline_thumbnails(
    window: Window,
    of: PathBuf,
    duration: usize,
    settings: State<'_, Configuration>,
) -> Result<TimelineThumbnailsResult, ()> {
    let hashed_path = &of.to_hashed();

    let folder_name = hashed_path.file_name().expect("Unable to get filename");

    let folder_path = PathBuf::from(settings.app_config.lock().unwrap().thumbnail_cache.clone())
        .join(folder_name);

    if folder_path.join(format!("{}.bmp", duration)).exists() {
        return Ok(TimelineThumbnailsResult::Found(folder_path));
    }

    let command = create_timeline_thumbnails_command(&of, &folder_path, &window.config()).unwrap();
    command
        .run(move |progress| {
            window
                .emit("thumbnail_progress", progress)
                .expect("Could not emit event");
        })
        .await
        .unwrap();

    return Ok(TimelineThumbnailsResult::Generating(folder_path));
}

fn friendly_error(error: anyhow::Error) -> String {
    error.to_string()
}

#[tauri::command]
pub async fn get_thumbnail(
    window: Window,
    of: &PathBuf,
    settings: &State<'_, Configuration>,
) -> Result<PathBuf, String> {
    info!("Getting thumbnail for: {}", of.display());

    let thumbnails_folder =
        PathBuf::from(settings.app_config.lock().unwrap().thumbnail_cache.clone());

    if !thumbnails_folder.exists() {
        info!("Creating folder: {}", thumbnails_folder.display());
        match fs::create_dir_all(&thumbnails_folder) {
            Ok(_) => {}
            Err(e) => {
                let error = format!("Unable to create thumbnail folder: {}", e.to_string());
                error!("{}", error);
                return Err(error);
            }
        }
    }

    let mut hash_name = of.to_hashed();
    hash_name.set_extension(THUMBNAIL_EXTENSION);

    let output_file_path = thumbnails_folder.join(hash_name.file_name().unwrap());

    if output_file_path.exists() {
        info!("Found thumbnail: {}", output_file_path.display());
        return Ok(output_file_path);
    }
    info!(
        "Thumbnail not found, creating: {}",
        output_file_path.display()
    );


    let command = match create_thumbnail_command(of, &output_file_path, &window.config()) {
        Ok(command) => command,
        Err(e) => {
            let error = format!("Unable to create thumbnail command: {}", e.to_string());
            error!("{}", error);
            return Err(error);
        }
    };

    command
        .run(|_| ())
        .await
        .map_err(friendly_error)?
        .wait()
        .map_err(|e| e.to_string())?;
    Ok(output_file_path)
}

#[tauri::command]
pub async fn get_latest_videos(
    window: Window,
    count: usize,
    settings: State<'_, Configuration>,
) -> Result<Vec<VideoData>, String> {
    let mut videos: Vec<VideoData> = Vec::new();
    let user_settings = settings.user_settings.lock().unwrap().clone();

    let from = PathBuf::from(&user_settings.videos_directory);
    let clip_location = PathBuf::from(&user_settings.clip_location);

    for video in find_latest_videos(&from, &clip_location)
        .into_iter()
        .take(count)
    {
        let video_path = &PathBuf::from(&video);
        let thumbnail = get_thumbnail(window.clone(), &video_path, &settings).await?;
        videos.push(VideoData {
            thumbnail: thumbnail.to_string_lossy().into(),
            file: video,
            name: video_path
                .file_name()
                .context("Unable to get filename")
                .unwrap()
                .to_string_lossy()
                .into(),
        });
    }
    Ok(videos)
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
