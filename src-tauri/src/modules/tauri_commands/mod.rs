use std::{path::PathBuf, fs};

use anyhow::{Context, Result};
use tauri::Window;

use super::{
    ffmpeg::{ get_version, installer::install_ffmpeg, ffmpeg_factory::create_thumbnail_command},
    file_processing::video_metadata::find_lastest_videos, config::{app_config::AppConfig, Static}, utils::filesystem_utils::PathBufExtensions,
};

#[derive(Clone, serde::Serialize)]
pub struct VideoData {
    thumbnail: String,
    file: String,
    name: String,
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
