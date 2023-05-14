use std::{thread, time};

use tauri::Window;

use super::{file_processing::video_metadata::find_lastest_videos, ffmpeg::{get_version, installer::install_ffmpeg}};



#[tauri::command]
pub async fn get_latest_videos(count: usize) -> Vec<String> {
    find_lastest_videos().into_iter().take(count).collect()
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
    }else {
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
        } ,
    }
}