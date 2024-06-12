use std::{path::PathBuf, process::Command};

use log::info;

use crate::modules::utils::command_utils::NoWindow;

pub mod ffmpeg;
pub mod ffmpeg_builder;
pub mod ffmpeg_builder_helpers;
pub mod ffmpeg_factory;
pub mod ffprobe;
pub mod models;
pub mod progress;

#[derive(Debug)]
pub enum VersionResultError {
    NotInstalled,
    ParseError,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Version {
    major: usize,
    minor: usize,
}

pub fn get_version(of: &str, ffmpeg_location: &PathBuf) -> Result<Version, VersionResultError> {
    #[cfg(target_os = "windows")]
    let full_program = ffmpeg_location.join("bin").join(of);
    #[cfg(target_os = "linux")]
    let full_program = of;

    info!("Checking version of {:?}", full_program);

    let output = Command::new(full_program)
        .arg("-version")
        .no_window()
        .output()
        .map_err(|_| VersionResultError::NotInstalled)?;
    let output_string = String::from_utf8_lossy(&output.stdout);
    let output_string_vec: Vec<&str> = output_string.split_whitespace().collect();

    let version_string = output_string_vec
        .get(2)
        .ok_or(VersionResultError::NotInstalled)?;
    let version_string_slit: Vec<&str> = version_string.split(['.', '-']).take(2).collect();

    fn try_get(vec: &Vec<&str>, index: usize) -> Result<usize, VersionResultError> {
        let x = vec.get(index).ok_or(VersionResultError::ParseError)?;
        x.parse::<usize>()
            .map_err(|_| VersionResultError::ParseError)
    }

    let result = Version {
        major: try_get(&version_string_slit, 0)?,
        minor: try_get(&version_string_slit, 1)?,
    };

    info!("Version of {:?} is {:?}", of, result);

    Ok(result)
}
