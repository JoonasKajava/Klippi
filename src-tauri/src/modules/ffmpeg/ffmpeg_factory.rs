use crate::modules::{utils::filesystem_utils::PathBufExtensions, config::{app_config::AppConfig, Static}};

use super::{
    ffmpeg_builder::{FFmpegBuilder, File, Param},
    models::clip_creation_options::ClipCreationOptions,
};
use anyhow::{Result, Context};
use std::{
    path::PathBuf, thread::ThreadId, fs,
};

pub fn create_timeline_thumbnails_command(from: &PathBuf, into: &PathBuf)-> Result<FFmpegBuilder> {
    // $ ffmpeg -hwaccel auto -skip_frame nokey -i Don\'t\ Starve\ Together\ 2021.01.04\ -\ 22.44.20.02.DVR.mp4 -r 1 -vf "scale=64:-1" -tune fastdecode -preset fast -benchmark test/out%d.bmp
    let mut instance = FFmpegBuilder::new();


    let thumbnails_folder= into.join("%d.bmp");

    fs::create_dir_all(&thumbnails_folder.parent().unwrap())?;

    instance.input(File { path: from.to_path_buf(), options: vec![
        Param::create_pair("hwaccel", "auto"),
        Param::create_pair("skip_frame", "nokey")
    ] })?
    .video_filter("scale", "64:-1")
    .output(File {
        path: thumbnails_folder,
        options: vec![
            Param::create_pair("r", "1"),
            Param::create_pair("tune", "fastdecode"),
            Param::create_pair("preset", "fast")
        ]
    });
    return Ok(instance);
}

pub fn create_thumbnail_command(of: &PathBuf, into: &PathBuf) -> Result<FFmpegBuilder> {
    let mut instance = FFmpegBuilder::new();

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

pub fn create_clip_command<'a>(options: &'a ClipCreationOptions) -> Result<FFmpegBuilder> {
    let mut instance = FFmpegBuilder::new();

    let framerate: f64;
    if options.speed < 1.0 {
        framerate = &options.framerate * &options.speed;
    } else {
        framerate = options.framerate;
    }


    instance
        .input(File {
            path: options.from.to_path_buf(),
            options: vec![
                Param::create_pair("ss", options.start.to_string()),
                Param::create_pair("to", options.end.to_string()),
                Param::create_pair("hwaccel", "auto"),
            ],
        })?
        .option(Param::Single("y".into()))
        .video_filter("scale", format!("-1:{}", options.resolution.to_string()))
        .video_filter("fps=fps", framerate.to_string())
        .set_speed(options.speed)
        .output(File {
            path: options.to.to_path_buf(),
            options: vec![
                Param::create_pair("b:v", format!("{}K", options.video_bitrate)),
                Param::create_pair("b:a", format!("{}K", options.audio_bitrate)),
                Param::create_pair("preset", "ultrafast"),
            ],
        });

    if options.mute {
        for input in &mut instance.inputs {
            input.options.push(Param::Single("an".into()))
        }
    }



    Ok(instance)
}
