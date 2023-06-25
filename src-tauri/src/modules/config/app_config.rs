

use serde::{Deserialize, Serialize};
use tauri::{
    Config, api::path::app_cache_dir,
};
use ts_rs::TS;

use super::{DefaultValues, FileConfig};

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/lib/models/")]
pub struct AppConfig {
    pub thumbnail_cache: String,
}

impl FileConfig for AppConfig {
    
}

impl DefaultValues for AppConfig {
    fn default(_config: &Config) -> Self {
        AppConfig {
            thumbnail_cache: app_cache_dir(&_config).expect("Unable to get app cache dir")
                .join("thumbnails")
                .to_string_lossy()
                .into(),
        }
    }
}