use std::{process::Command, path::PathBuf};

use crate::modules::config::{app_config::AppConfig, Static};

pub mod ffprobe;
pub mod installer;
pub mod ffmpeg;
pub mod ffmpeg_builder;
pub mod ffmpeg_factory;
pub mod models;
pub mod progress;
pub mod ffmpeg_builder_helpers;

#[derive(Debug)]
pub enum VersionResultError {
    NotInstalled,
    ParseError
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Version {
    major: usize,
    minor: usize
}

pub fn get_version(of: &str) -> Result<Version, VersionResultError> {

    let ffmpeg_location = PathBuf::from(AppConfig::current().ffmpeg_location.clone());

    let full_program = ffmpeg_location.join("bin").join(&of);

    println!("Checking version of {:?}", full_program);

    let output = Command::new( full_program)
        .arg("-version")
        .output().map_err(|_|VersionResultError::NotInstalled)?;
    let output_string = String::from_utf8_lossy(&output.stdout);
    let output_string_vec: Vec<&str> = output_string.split_whitespace().collect();

    let version_string = output_string_vec.get(2).ok_or(VersionResultError::NotInstalled)?;
    let version_string_slit:Vec<&str> = version_string.split(['.', '-']).take(2).collect();

    fn try_get(vec: &Vec<&str>, index: usize) -> Result<usize, VersionResultError> {
        let x = vec.get(index).ok_or(VersionResultError::ParseError)?;
        Ok(x.parse::<usize>().map_err(|_| VersionResultError::ParseError)?)
    }

    let result = Version {
        major: try_get(&version_string_slit, 0)?,
        minor: try_get(&version_string_slit, 1)?
    };

    Ok(result)
}
