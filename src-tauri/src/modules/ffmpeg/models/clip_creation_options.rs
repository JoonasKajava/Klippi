use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Default, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../src/lib/models/")]
pub struct ClipCreationOptions {
    pub from: PathBuf,
    pub to: PathBuf,
    pub start: Option<f64>,
    pub end: Option<f64>,

    pub video_bitrate: f64,
    pub audio_bitrate: f64,
    pub framerate: f64,
    pub speed: f64,
    pub resolution: u32,
    pub mute: bool,
    pub format: String,
    pub preset: String,
}
