use super::ffmpeg_builder::{FFmpegBuilder, File, Param};
use anyhow::{Result};
use std::path::PathBuf;

pub fn create_thumbnail_command<'a>(of: &'a PathBuf, into: &'a PathBuf) -> Result<FFmpegBuilder<'a>> {
    let instance = FFmpegBuilder::new()
        .input(File {
            path: of,
            options: vec![Param::Pair("sseof", "-3")],
        })?
        .output(File { path: into, options: vec![
            Param::Pair("preset", "ultrafast"),
            Param::Pair("q:v", "5"),
            Param::Pair("update", "1"),
            Param::Pair("vf", "scale=240:-1")
            ] });

    Ok(instance)
}
