use std::path::PathBuf;

use anyhow::Context;
use tauri::{Window, State};

use crate::modules::{config::Configuration, file_processing::video_metadata::find_latest_videos};

use super::{VideoData, thumbnail::get_thumbnail};



#[tauri::command]
pub async fn discover_videos(
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
        let thumbnail = get_thumbnail(window.clone(), video_path, &settings).await?;
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