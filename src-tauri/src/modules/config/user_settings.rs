use serde::{Deserialize, Serialize};
use ts_rs::TS;
use std::path::{PathBuf};
use tauri::api::path::{home_dir};
use tauri::api::path::video_dir;
use tauri::Config;

use super::{FileConfig, DefaultValues};

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to="../src/lib/models/")]
pub struct UserSettings {
    pub clip_location: String,
    pub videos_directory: String,
}

impl FileConfig for UserSettings {
    
}

impl DefaultValues for UserSettings {
    fn default(_config: &Config) -> UserSettings {
        UserSettings { 
            clip_location: get_video_dir().join("Clips").into_os_string().into_string().unwrap(),
            videos_directory: get_video_dir().into_os_string().into_string().unwrap()
        }
    }
}

fn get_video_dir() -> PathBuf {
    match video_dir() {
        Some(dir) => return dir,
        None => return home_dir().expect("Unable to get home dir"),
    }
}