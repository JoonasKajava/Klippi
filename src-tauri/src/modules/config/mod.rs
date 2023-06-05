pub mod user_settings;
pub mod app_config;



#[derive(Debug)]
pub struct Configuration {
    pub app_config: Mutex<AppConfig>,
    pub user_settings: Mutex<UserSettings>,
}

impl Init for Configuration {
    fn init(config: &Config) -> Self {
        Configuration {
            app_config: Mutex::new(AppConfig::init(config)),
            user_settings: Mutex::new(UserSettings::init(config)),
        }
    }
}



use std::{sync::{Mutex}, path::PathBuf, fs::{File, self}};

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use tauri::Config;

use self::{user_settings::UserSettings, app_config::AppConfig};

pub trait Init {
    fn init(config: &Config) -> Self;
}

pub trait Save {
    fn save(&self, config: &Config) -> Result<()>;
}

pub trait DefaultValues {
    fn default(config: &Config) -> Self;
}

pub trait JsonConfig : Save + Init + DefaultValues {
    
    fn file_location(config: &Config) -> Option<PathBuf>;
}

impl <T: JsonConfig+ for<'de> Deserialize<'de>> Init for T {
    fn init(config: &tauri::Config) -> Self {
        let app_config_file =
            T::file_location(config).expect("Unable to find app config file");

        if !app_config_file.try_exists().unwrap_or(false) {
            let default = T::default(config); 
            let _ = &default.save(&config).expect("Unable to save config");
            return default; 
        } else {
            let config_reader = File::open(app_config_file).expect("Unable to open config file");
            match serde_json::from_reader::<File, Self>(config_reader) {
                Ok(x) => return x,
                Err(_) => {
                    return T::default(config);
                }
            }
        }
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