use serde::{Deserialize, Serialize};
use std::fs::{File, self};
use std::path::{PathBuf};
use std::sync::{Arc, RwLock};
use tauri::api::path::{app_config_dir, home_dir};
use tauri::api::path::video_dir;
use tauri::Config;
use anyhow::Result;

use super::{JsonConfig, DefaultValues, Static};
use super::app_config::AppConfig;

const USER_SETTINGS_FILENAME: &str = "UserSettings.json";

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
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
    fn default(config: &Config) -> Result<UserSettings> {
        Ok(UserSettings { 
            clip_location: get_video_dir().join("Clips").into_os_string().into_string().unwrap(),
            videos_directory: get_video_dir().into_os_string().into_string().unwrap()
        })
    }
}

impl Static for UserSettings {
    fn current() -> Arc<Self> {
        USER_SETTINGS.read().unwrap().clone()
    }

    fn make_current(self) {
        *USER_SETTINGS.write().unwrap() = Arc::new(self)
    }
}

lazy_static::lazy_static! {
    static ref USER_SETTINGS: RwLock<Arc<UserSettings>> = RwLock::new(Default::default());
}


fn get_video_dir() -> PathBuf {
    match video_dir() {
        Some(dir) => return dir,
        None => return home_dir().expect("Unable to get home dir"),
    }
}