use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use ts_rs::TS;

use super::{DefaultValues, FileConfig};

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../src/lib/models/")]
pub struct UserSettings {
    pub clip_location: String,
    pub videos_directory: String,
}

impl FileConfig for UserSettings {}

impl DefaultValues for UserSettings {
    fn default(app: &AppHandle) -> UserSettings {
        UserSettings {
            clip_location: get_video_dir(app)
                .join("Clips")
                .into_os_string()
                .into_string()
                .unwrap(),
            videos_directory: get_video_dir(app).into_os_string().into_string().unwrap(),
        }
    }
}

fn get_video_dir(app: &AppHandle) -> PathBuf {
    match app.path().video_dir() {
        Ok(dir) => dir,
        Err(_) => app.path().home_dir().expect("Unable to get home dir"),
    }
}
