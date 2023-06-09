use std::{path::PathBuf, process::Command};
use anyhow::{Result, bail};
use tauri::{Config, api::path::app_data_dir};
use std::fmt;

use crate::modules::utils::command_utils::NoWindow;

#[derive(Debug)]
pub struct FFmpegBuilder {

    pub options: Vec<Param>,
    pub inputs: Vec<File>,
    pub outputs: Vec<File>,
    pub video_filters: Vec<Param>,
    pub audio_filters: Vec<Param>,
    pub working_directory : Option<PathBuf>

}

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub options: Vec<Param>
}

#[derive(Debug)]
pub enum Param {
    Single(String),
    Pair(String, String)
}

impl Param {
    pub fn create_pair(key: impl Into<String>, value: impl Into<String>) -> Param {
        Param::Pair(key.into(), value.into())
    }
}

impl<'a> FFmpegBuilder {

    pub fn new(config: &Config) -> FFmpegBuilder {

        #[cfg(target_os = "windows")]
        let working_directory = Some(app_data_dir(config).expect("Unable to get app data dir").join("ffmpeg"));
        #[cfg(target_os = "linux")]
        let working_directory = None;

        FFmpegBuilder { options: Vec::new(), inputs: Vec::new(), outputs: Vec::new(), video_filters: vec![], audio_filters: vec![], working_directory }
    }

    pub fn option(&mut self, option:Param) -> &mut Self {
        self.options.push(option);
        self
    }

    pub fn input(&mut self, input: File) -> Result<&mut Self> {
        if !input.path.try_exists()? {
            bail!("Trying to add file({}) that doesn't exist into ffmpeg", input.path.display());
        }
        self.inputs.push(input);
        Ok(self)
    }

    pub fn video_filter(&mut self, key:impl Into<String>, value: impl Into<String>) ->&mut Self {
        self.video_filters.push(Param::Pair(key.into(), value.into()));
        self
    }

    pub fn audio_filter(&mut self, key:impl Into<String>, value: impl Into<String>) ->&mut Self {
        self.audio_filters.push(Param::Pair(key.into(), value.into()));
        self
    }

    pub fn output(&mut self, output: File) -> &mut Self {
        self.outputs.push(output);
        self
    }

    fn create_filter_string(&self, filters: &Vec<Param>) -> String {
        let filters: Vec<String> = filters.iter().map(|x| {
            match x {
                Param::Single(s) => s.clone(),
                Param::Pair(key, value) => format!("{}={}", key, value),
            }
        }).collect();
        filters.join(", ")
    }

    pub fn to_command(&self) -> Command {
        let mut command = Command::new("ffmpeg");
        
        match &self.working_directory {
            Some(path) => command.current_dir(path),
            None => &mut command,
        };

        for option in &self.options {
            option.push_to(&mut command);
        }

        for input in &self.inputs {
            input.push_to(&mut command, true);
        }



        if !self.video_filters.is_empty() {
            command.args(["-vf", self.create_filter_string(&self.video_filters).as_str()]);
        }
        if !self.audio_filters.is_empty() {
            command.args(["-filter:a", self.create_filter_string(&self.audio_filters).as_str()]);
        }

        for output in &self.outputs {
            output.push_to(&mut command, false);
        }

        command.no_window();


        command
    }
}

impl<'a> fmt::Display for FFmpegBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = format!("{:?}", self.clone().to_command());
        f.write_str(str.as_str())
        
    }
}

#[allow(dead_code)]
impl<'a> File {
    pub fn from(path: &PathBuf) -> File {
        File { path: path.into(), options: Vec::new() }
    }

    pub fn option(mut self, option: Param) -> Self{
        self.options.push(option);
        self
    }

    pub fn push_to(&self, command: &mut Command, is_input: bool) {
        for option in &self.options  {
            option.push_to(command);
        }

        if is_input  {
            command.arg("-i");
        }
        command.arg(self.path.to_str().unwrap());
    }
}

impl Param {
    fn push_to(&self, command: &mut Command) {
        match &self {
            Param::Single(arg) => command.arg("-".to_owned() + arg),
            Param::Pair(arg, value) => command.args(["-".to_owned()+arg, value.to_string()]),
        };
    }
}