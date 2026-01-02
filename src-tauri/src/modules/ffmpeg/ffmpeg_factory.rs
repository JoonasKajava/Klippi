use std::{fs, path::PathBuf};

use anyhow::Result;
use tauri::{AppHandle, Config};

use super::{
    ffmpeg_builder::{FFmpegBuilder, File, Param},
    models::clip_creation_options::ClipCreationOptions,
};

pub fn create_timeline_thumbnails_command(
    from: &PathBuf,
    into: &PathBuf,
    app: &AppHandle,
) -> Result<FFmpegBuilder> {
    let mut instance = FFmpegBuilder::new(app);

    let thumbnails_folder = into.join("%d.bmp");

    fs::create_dir_all(thumbnails_folder.parent().unwrap())?;

    instance
        .input(File {
            path: from.to_path_buf(),
            options: vec![
                Param::create_pair("hwaccel", "auto"),
                Param::create_pair("skip_frame", "nokey"),
            ],
        })?
        .video_filter("scale", "64:-1")
        .output(File {
            path: thumbnails_folder,
            options: vec![
                Param::create_pair("r", "1"),
                Param::create_pair("tune", "fastdecode"),
                Param::create_pair("preset", "fast"),
            ],
        });
    Ok(instance)
}

pub fn create_thumbnail_command(
    of: &PathBuf,
    into: &PathBuf,
    app: &AppHandle,
) -> Result<FFmpegBuilder> {
    let mut instance = FFmpegBuilder::new(app);

    instance
        .input(File {
            path: of.to_path_buf(),
            options: vec![Param::create_pair("sseof", "-3")],
        })?
        .output(File {
            path: into.to_path_buf(),
            options: vec![
                Param::create_pair("preset", "ultrafast"),
                Param::create_pair("q:v", "5"),
                Param::create_pair("update", "1"),
                Param::create_pair("vf", "scale=240:-1"),
            ],
        });

    Ok(instance)
}

pub fn create_clip_command(
    options: &ClipCreationOptions,
    app: &AppHandle,
) -> Result<FFmpegBuilder> {
    let mut instance = FFmpegBuilder::new(app);

    let framerate: f64;
    if options.speed < 1.0 {
        framerate = &options.framerate * &options.speed;
    } else {
        framerate = options.framerate;
    }

    let mut input_file_options = vec![Param::create_pair("hwaccel", "auto")];

    if options.start.is_some() {
        input_file_options.push(Param::create_pair("ss", options.start.unwrap().to_string()));
    }
    if options.end.is_some() {
        input_file_options.push(Param::create_pair("to", options.end.unwrap().to_string()));
    }
    instance
        .input(File {
            path: options.from.to_path_buf(),
            options: input_file_options,
        })?
        .option(Param::Single("y".into()))
        .video_filter("scale", format!("-1:{}", options.resolution))
        .video_filter("fps=fps", framerate.to_string())
        .set_speed(options.speed)
        .output(File {
            path: options.to.to_path_buf(),
            options: vec![
                Param::create_pair("f", options.format.to_string()),
                Param::create_pair("b:v", format!("{}K", options.video_bitrate)),
                Param::create_pair("b:a", format!("{}K", options.audio_bitrate)),
                Param::create_pair("preset", options.preset.to_string()),
            ],
        });

    if options.mute {
        for input in &mut instance.inputs {
            input.options.push(Param::Single("an".into()))
        }
    }

    Ok(instance)
}
