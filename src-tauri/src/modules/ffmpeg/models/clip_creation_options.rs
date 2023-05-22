use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use ts_rs::TS;


#[derive(Debug, Default, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to="../src/models/")]
pub struct ClipCreationOptions {
    pub from: PathBuf,
    pub to: PathBuf,
    pub start: f64,
    pub end: f64,

    pub video_bitrate: f64,
    pub audio_bitrate: f64,
    pub framerate: f64,
    pub speed: f64,
    pub resolution: u32,
    pub mute: bool
}