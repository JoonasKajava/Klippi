use anyhow::{Context, Result};
use std::{process::{Child, Stdio},  thread, io::{BufReader, BufRead}};


use crate::modules::ffmpeg::progress::{self, Progress, Status};

use super::ffmpeg_builder::{FFmpegBuilder, Param};

pub struct FFmpegRunner {

    pub process: Child
}


impl<'a> FFmpegBuilder<'a>  {
    pub async fn run(mut self, on_progress: impl Fn(Progress) + std::marker::Sync + std::marker::Send + 'static) -> Result<Child> {
        self = self.option(Param::Pair("progress", "-"));
        self = self.option(Param::Single("nostats"));

        println!("Running: {}", self);

        let mut child = self.to_command().stdout(Stdio::piped()).spawn()?;

        let stdout = child.stdout.take().context("Unable to get stdout")?;

        thread::spawn(move || {
            let mut bufread = BufReader::new(stdout);
            let mut buf = String::new();
            let mut progress: Progress = Default::default();
            while let Ok(n) = bufread.read_line(&mut buf) {
                if n > 0 {
                    match progress.try_parse(buf.as_str()) {
                        Ok(_) => on_progress(progress.clone()),
                        Err(e) => println!("Parse error: {:?}", e),
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
        println!("pääsi tänne");
        Ok(child)

    }
}