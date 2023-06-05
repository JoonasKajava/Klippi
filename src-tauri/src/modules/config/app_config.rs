use std::{
    env::temp_dir,
    path::PathBuf
};

use serde::{Deserialize, Serialize};
use tauri::{
    api::path::{app_config_dir, app_data_dir},
    Config,
};
use ts_rs::TS;

use super::{DefaultValues, JsonConfig};

const APP_CONFIGURATION_FILENAME: &str = "AppConfig.json";

#[derive(Debug, Default, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/lib/models/")]
pub struct AppConfig {
    pub ffmpeg_location: String,
    pub thumbnail_cache: String,
}

impl JsonConfig for AppConfig {
    fn file_location(config: &Config) -> Option<PathBuf> {
        Some(app_config_dir(config)?.join(APP_CONFIGURATION_FILENAME))
    }
}

impl DefaultValues for AppConfig {
    fn default(config: &Config) -> Self {
        AppConfig {
            ffmpeg_location: app_data_dir(config)
                .expect("Unable to get app data dir")
                .join("ffmpeg")
                .to_string_lossy()
                .into(),
            thumbnail_cache: temp_dir()
                .join("klippi_thumbnails")
                .to_string_lossy()
                .into(),
        }
    }
}