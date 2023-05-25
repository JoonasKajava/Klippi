use std::{
    env::temp_dir,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tauri::{
    api::path::{app_cache_dir, app_config_dir, app_data_dir},
    Config,
};
use ts_rs::TS;

use super::{DefaultValues, JsonConfig, Static};

const APP_CONFIGURATION_FILENAME: &str = "AppConfig.json";

#[derive(Debug, Default, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/models/")]
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
    fn default(config: &Config) -> Result<AppConfig> {
        Ok(AppConfig {
            ffmpeg_location: app_data_dir(config)
                .context("Unable to get app data dir")?
                .join("ffmpeg")
                .to_string_lossy()
                .into(),
            thumbnail_cache: temp_dir()
                .join("klippi_thumbnails")
                .to_string_lossy()
                .into(),
        })
    }
}

impl Static for AppConfig {
    fn current() -> Arc<Self> {
        APP_CONFIG.read().expect("Cannot read APP_CONFIG").clone()
    }

    fn make_current(self) {
        *APP_CONFIG.write().expect("APP_CONFIG was not writable") = Arc::new(self)
    }
}

lazy_static::lazy_static! {
    static ref APP_CONFIG: RwLock<Arc<AppConfig>> = RwLock::new(Default::default());
}
