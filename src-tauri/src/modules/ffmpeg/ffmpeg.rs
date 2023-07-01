use anyhow::{Context, Result};
use log::{warn, info};
use tauri::{Config, api::path::app_data_dir};
use std::{process::{Child, Stdio},  thread, io::{BufReader, BufRead}, path::PathBuf};


use crate::modules::ffmpeg::progress::{Progress, Status};

use super::ffmpeg_builder::{FFmpegBuilder, Param};

pub fn get_ffmpeg_location(config: &Config) -> PathBuf {
    app_data_dir(config).expect("Unable to get app data dir").join("ffmpeg")
}


impl FFmpegBuilder {
    pub async fn run(mut self, on_progress: impl Fn(Progress) + std::marker::Sync + std::marker::Send + 'static) -> Result<Child> {
        self.option(Param::create_pair("progress", "-"));
        self.option(Param::Single("nostats".into()));

        info!("Running: {}", self);
        
        let mut child = self.to_command().stdout(Stdio::piped()).spawn()?;

        let stdout = child.stdout.take().context("Unable to get stdout")?;

        thread::spawn(move || {
            let mut bufread = BufReader::new(stdout);
            let mut buf = String::new();
            let mut progress: Progress = Default::default();
            while let Ok(n) = bufread.read_line(&mut buf) {
                if n > 0 {
                    let line = buf.as_str();
                    info!("{}", line);
                    match progress.try_parse(line) {
                        Ok(_) => on_progress(progress.clone()),
                        Err(e) => warn!("Parse error: {:?}", e),
                    };

                    buf.clear();

                    if progress.status == Status::End {
                        break;
                    }
                } else {
                    break;
                }
            }
        });
        Ok(child)

    }
}