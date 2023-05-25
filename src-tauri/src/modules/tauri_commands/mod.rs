use std::{path::PathBuf, fs, sync::{Arc}, process::Command};
use anyhow::{Context, Result};
use tauri::{Window};
use ts_rs::TS;

use crate::modules::config::user_settings::UserSettings;

use super::{
    ffmpeg::{ get_version, installer::install_ffmpeg, ffmpeg_factory::{create_thumbnail_command, create_clip_command, create_timeline_thumbnails_command}, models::clip_creation_options::ClipCreationOptions, progress::{Progress, Status}},
    file_processing::video_metadata::find_lastest_videos, config::{app_config::AppConfig, Static}, utils::filesystem_utils::PathBufExtensions,
};


#[derive(Clone, serde::Serialize, TS)]
#[ts(export, export_to="../src/models/")]
pub struct VideoData {
    thumbnail: String,
    file: String,
    name: String,
}

#[tauri::command]
pub async fn clip_exists(file: PathBuf) -> bool {
    
    PathBuf::from(&UserSettings::current().clip_location).join(file).exists()
}

#[derive(Clone, serde::Serialize, TS)]
#[ts(export, export_to="../src/models/")]
pub enum TimelineThumbnailsResult {
    Generating(PathBuf),
    Found(PathBuf)
}

#[tauri::command]
pub async fn get_timeline_thumbnails(window: Window,of: PathBuf, duration: usize) -> TimelineThumbnailsResult {
    let hashed_path = &of.to_hashed();

    let folder_name = hashed_path.file_name().expect("Unable to get filename");

    let folder_path = PathBuf::from(AppConfig::current().thumbnail_cache.clone()).join(folder_name);

    if folder_path.join(format!("{}.bmp", duration)).exists() {
        return TimelineThumbnailsResult::Found(folder_path);
    }

    let command = create_timeline_thumbnails_command(&of, &folder_path).unwrap();
    command.run(move |progress| {
        window.emit("thumbnail_progress", progress).expect("Could not emit event");
    }).await.unwrap();

    return TimelineThumbnailsResult::Generating(folder_path)
}

#[tauri::command]
pub async fn get_thumbnail(of: &PathBuf) -> Result<PathBuf> {

    let thumbnails_folder = PathBuf::from(AppConfig::current().thumbnail_cache.clone());
    if !thumbnails_folder.exists() {
        fs::create_dir_all(&thumbnails_folder)?;
    }
    
    let mut hash_name = of.to_hashed();
    hash_name.set_extension("jpg");

    let output_file_path = thumbnails_folder.join(hash_name.file_name().unwrap());

    if output_file_path.exists() {
        println!("Found thumbnail: {}", output_file_path.display());
        return Ok(output_file_path);
    }

    create_thumbnail_command(of, &output_file_path)?.run(|_|()).await?.wait()?;
    Ok(output_file_path)
}

#[tauri::command]
pub async fn get_latest_videos(count: usize) -> Vec<VideoData> {
    let mut videos: Vec<VideoData> = Vec::new();
    for video in find_lastest_videos().into_iter().take(count) {
        let video_path = &PathBuf::from(&video);
        let thumbnail = get_thumbnail(&video_path).await.expect("Could not get thumbnail");
        videos.push(VideoData {
            thumbnail: thumbnail.to_string_lossy().into(),
            file: video,
            name: video_path.file_name().context("Unable to get filename").unwrap().to_string_lossy().into()
        });
    }
    videos
}

#[tauri::command]
pub async fn verify_dependencies() -> Result<(), Vec<String>> {
    let mut failed_dependencies: Vec<String> = Vec::new();
    if get_version("ffmpeg").is_err() {
        failed_dependencies.push("ffmpeg".into());
    }

    if get_version("ffprobe").is_err() {
        failed_dependencies.push("ffprobe".into());
    }

    if failed_dependencies.is_empty() {
        Ok(())
    } else {
        Err(failed_dependencies)
    }
}
#[tauri::command]
pub async fn install_dependencies(window: Window, path: &str) -> Result<String, String> {
    let result = install_ffmpeg(window, &path).await;
    match result {
        Ok(_) => Ok("Successful".into()),
        Err(e) => {
            println!("{:?}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn create_clip(window: Window,options: ClipCreationOptions) -> Result<String, String>  {

    let final_options = ClipCreationOptions {
        to: PathBuf::from(UserSettings::current().clip_location.clone()).join(options.to),
        ..options
    };

    let command = create_clip_command(&final_options).map_err(|_|"Unable to get command".to_string())?;
    
    command.run(move |progress| {
        window.emit("ffmpeg_progress", progress).expect("Could not emit event");
    }).await.map_err(|_|"Running failed!")?.wait().map_err(|_|"Wait failed")?;

    final_options.to.reveal();

    Ok("Success".into())
}

#[tauri::command]
pub async fn get_user_settings() -> Arc<UserSettings> {
     return UserSettings::current().clone();
}