pub mod user_settings;
pub mod app_config;

use std::{sync::Arc, path::PathBuf, fs::{File, self}};

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use tauri::Config;

pub trait Init {
    fn init(config: &Config) -> Result<()>;
}

pub trait Save {
    fn save(&self, config: &Config) -> Result<()>;
}

pub trait DefaultValues {
    fn default(config: &Config) -> Result<Self> where Self: Sized;
}

pub trait JsonConfig : Save + Init + Static + DefaultValues {
    
    fn file_location(config: &Config) -> Option<PathBuf>;
}

impl <T: JsonConfig+ for<'de> Deserialize<'de>> Init for T {
    fn init(config: &tauri::Config) -> anyhow::Result<()> {
        let app_config_file =
            T::file_location(config).context("Unable to find app config file")?;

        if !app_config_file.try_exists().unwrap_or(false) {
            T::default(config)?.save(&config)?
        } else {
            let config_reader = File::open(app_config_file)?;
            match serde_json::from_reader::<File, Self>(config_reader) {
                Ok(x) => x.make_current(),
                Err(_) => {
                    let _sink = T::default(config)?;
                }
            }
        }

        Ok(())
    }
}

impl <T: JsonConfig + Serialize> Save for T {
    fn save(&self, config: &Config) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;

        let file_location = T::file_location(config).context("Unable to get file location")?;

        let config_dir = file_location.parent().context("Unable to get config location")?;

        fs::create_dir_all(config_dir)?;

        fs::write(file_location, json)?;

        Ok(())
    }
}

pub trait Static {
    fn current() -> Arc<Self>;
    fn make_current(self);
}