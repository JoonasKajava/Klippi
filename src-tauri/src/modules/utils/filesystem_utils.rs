use std::{path::PathBuf, process::Command};
use log::info;
use md5;

use crate::modules::utils::command_utils::NoWindow;


pub trait PathBufExtensions {
    fn to_hashed(&self) -> PathBuf;

    fn reveal(&self);
}

impl PathBufExtensions for PathBuf {
    fn to_hashed(&self) -> PathBuf {
        let mut new_path = self.clone();

        let digest = md5::compute(self.to_str().unwrap());
        let hash = format!("{:x}", digest);

        new_path.set_file_name(hash);
        return new_path;
    }

    fn reveal(&self) {
        info!("Revealing file: {}", self.to_str().unwrap());
        let mut command = Command::new("explorer");
        command.no_window();
        command.args(["/select,", self.to_str().unwrap()]);
        println!("{:?}", command);
        command.spawn().unwrap();
    }
}