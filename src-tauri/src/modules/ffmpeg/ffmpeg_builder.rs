use std::{path::{PathBuf, Path}, process::Command};
use anyhow::{Result, bail, Context};
use std::fmt;

use crate::modules::config::{app_config::AppConfig, Static};



#[derive(Debug)]
pub struct FFmpegBuilder<'a> {

    pub options: Vec<Param<'a>>,
    pub inputs: Vec<File<'a>>,
    pub outputs: Vec<File<'a>>,
    pub command: &'a str

}

#[derive(Debug)]
pub struct File<'a> {
    pub path: &'a PathBuf,
    pub options: Vec<Param<'a>>
}

#[derive(Debug)]
pub enum Param<'a> {
    Single(&'a str),
    Pair(&'a str, &'a str)
}

impl<'a> FFmpegBuilder<'a> {
    pub fn get_full_command(command: &str) -> PathBuf{
        PathBuf::from(AppConfig::current().ffmpeg_location.clone())
        .join("bin")
        .join(command)
    }

    pub fn new() -> FFmpegBuilder<'a> {
        FFmpegBuilder { options: Vec::new(), inputs: Vec::new(), outputs: Vec::new(), command: "ffmpeg" }
    }

    pub fn option(mut self, option:Param<'a>) -> Self {
        self.options.push(option);
        self
    }

    pub fn input(mut self, input: File<'a>) -> Result<Self> {
        if !input.path.try_exists()? {
            bail!("Trying to add file({}) that doesn't exist into ffmpeg", input.path.display());
        }
        self.inputs.push(input);
        Ok(self)
    }

    pub fn output(mut self, output: File<'a>) -> Self {
        self.outputs.push(output);
        self
    }

    pub fn to_command(&self) -> Command {
        let mut command = Command::new(FFmpegBuilder::get_full_command(self.command));

        for option in &self.options {
            option.push_to(&mut command);
        }

        for input in &self.inputs {
            input.push_to(&mut command, true);
        }

        for output in &self.outputs {
            output.push_to(&mut command, false);
        }

        command
    }
}

impl<'a> fmt::Display for FFmpegBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = format!("{:?}", self.clone().to_command());
        f.write_str(&str.as_str())
        
    }
}


impl<'a> File<'a> {
    pub fn from(path: &PathBuf) -> File {
        File { path: path, options: Vec::new() }
    }

    pub fn option(mut self, option: Param<'a>) -> Self{
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
        command.arg(&self.path.to_str().unwrap());
    }
}

impl Param<'_> {
    fn push_to(&self, command: &mut Command) {
        match &self {
            Param::Single(arg) => command.arg("-".to_owned() + arg),
            Param::Pair(arg, value) => command.args(["-".to_owned()+arg, value.to_string()]),
        };
    }
}