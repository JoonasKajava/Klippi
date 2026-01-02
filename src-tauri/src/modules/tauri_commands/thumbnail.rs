use std::{path::PathBuf, fs};

use log::{info, error};
use tauri::{Window, State, Manager};

use crate::modules::{config::{Configuration, constants::{THUMBNAIL_EXTENSION, SUPPORTED_VIDEO_EXTENSIONS}}, utils::filesystem_utils::PathBufExtensions, ffmpeg::ffmpeg_factory::{create_thumbnail_command, create_timeline_thumbnails_command}};

use super::TimelineThumbnailsResult;

#[tauri::command]
pub async fn get_supported_video_extensions() -> Vec<String> {
    SUPPORTED_VIDEO_EXTENSIONS
        .iter()
        .map(|x| x.to_string())
        .collect()
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
                let error = format!("Unable to create thumbnail folder: {}", e);
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
            let error = format!("Unable to create thumbnail command: {}", e);
            error!("{}", error);
            return Err(error);
        }
    };

    command
        .run(|_| ())
        .await
        .map_err(|e| e.to_string())?
        .wait()
        .map_err(|e| e.to_string())?;
    Ok(output_file_path)
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

    Ok(TimelineThumbnailsResult::Generating(folder_path))
}