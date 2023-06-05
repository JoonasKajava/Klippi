use serde::{Deserialize, Serialize};
use ts_rs::TS;
use std::path::{PathBuf};
use std::sync::{Arc, RwLock};
use tauri::api::path::{app_config_dir, home_dir};
use tauri::api::path::video_dir;
use tauri::Config;
use anyhow::Result;

use super::{JsonConfig, DefaultValues};

const USER_SETTINGS_FILENAME: &str = "UserSettings.json";

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to="../src/lib/models/")]
pub struct UserSettings {
    pub clip_location: String,
    pub videos_directory: String,
}

impl JsonConfig for UserSettings {
    fn file_location(config: &Config) -> Option<PathBuf> {
        Some(app_config_dir(config)?.join(USER_SETTINGS_FILENAME))
    }
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