use std::{time::Duration};
use anyhow::{Result};
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Default, Clone, TS, Serialize)]
#[ts(export, export_to="../src/models/")]
pub struct Progress {
    /// What frame ffmpeg is on.
    pub frame: Option<u64>,
    /// What framerate ffmpeg is processing at.
    pub fps: Option<f64>,
    /// How much data ffmpeg has output so far, in bytes.
    pub total_size: Option<u64>,
    /// How far ffmpeg has processed.
    #[ts(type = "number | null")]
    pub out_time: Option<f64>,
    /// How many frames were duplicated? The meaning is unclear.
    pub dup_frames: Option<u64>,
    /// How many frames were dropped.
    pub drop_frames: Option<u64>,
    /// How fast it is processing, relative to 1x playback speed.
    pub speed: Option<f64>,
    pub status: Status
}


impl Progress {
    fn parse_value<T : std::str::FromStr>(value: &str) -> Result<T, ProgressParseError> {
        value.trim().parse::<T>().or(Err(ProgressParseError::UnableToParseValue(value.to_string())))
    }

    fn set(&mut self, key: &str, value: &str) -> Result<(), ProgressParseError> {
        match key {
            "frame" => self.frame = Some(Progress::parse_value(value)?),
            "fps" => self.fps = Some(Progress::parse_value(value)?),
            "total_size" => self.total_size = Some(Progress::parse_value(value)?),
            "out_time_us" => self.out_time = Some(Duration::from_micros(Progress::parse_value(value)?).as_secs_f64()),
            "out_time_ms" => (),
            "out_time" => (),
            "bitrate" => (),
            "stream_0_0_q" => (),
            "dup_frames" => self.dup_frames = Some(Progress::parse_value(value)?),
            "drop_frames" => self.drop_frames = Some(Progress::parse_value(value)?),
            "speed" => self.speed = Some(Progress::parse_value(value.trim_end_matches('x'))?),
            "progress" => self.status = match value {
                "continue" => Status::Continue,
                "end" => Status::End,
                _ => Status::Unknown
            },
            _ => {
                return Err(ProgressParseError::UnknownKey(format!("{}={}", key, value)));
            } 
        }
        Ok(())
    }

    pub fn try_parse(&mut self, str: &str) -> Result<(),ProgressParseError> {
        let str_timmed = str.trim();
        let mut key_value_split = str_timmed.splitn(2, '=');

        let key = key_value_split.next().ok_or(ProgressParseError::KeyNotFound)?;
        let value = key_value_split.next().ok_or(ProgressParseError::ValueNotFound)?;
        self.set(key, value)?;
        Ok(())
    } 
}


#[derive(Debug)]
pub enum ProgressParseError {
    KeyNotFound,
    ValueNotFound,
    UnableToParseValue(String),
    UnknownKey(String)
}
#[derive(Debug, Default, PartialEq, Clone, Serialize, TS)]
#[ts(export, export_to="../src/models/")]
pub enum Status {
    Continue,
    End,
    #[default] Unknown
}