use serde::{Deserialize, Serialize};
use std::fs::{File, self};
use std::path::{PathBuf, Path};
use std::sync::{Arc, RwLock};
use tauri::api::path::app_config_dir;
use tauri::api::path::video_dir;
use tauri::Config;

const USER_SETTINGS_FILENAME: &str = "UserSettings.json";

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub clip_location: String,
    pub videos_directory: String,
}

impl UserSettings {
    fn file_location(config: &Config) -> Option<PathBuf> {
        app_config_dir(config)
    }
    
    fn default(_config: &Config) -> Result<UserSettings, String> {
        Ok(UserSettings { 
            clip_location: video_dir().expect("Able to get video directory").join("Clips").into_os_string().into_string().unwrap(),
            videos_directory: video_dir().expect("Able to get video directory").into_os_string().into_string().unwrap()
        })
    }

    fn create_default(config: &Config) -> Result<(), String> {
        let default = UserSettings::default(&config)?;
        default.clone().make_current();
        UserSettings::save(&config)?;
        Ok(())
    }

    pub fn load(config: &Config) -> Result<(), String> {
        let config_file = UserSettings::file_location(config).ok_or("Unable to get user settings file location")?.join(USER_SETTINGS_FILENAME);

        if !&config_file.exists() {
            let _sink = UserSettings::create_default(&config)?;
        } else {
            let config_reader = fs::File::open(config_file).map_err(|e| e.to_string())?;
            match serde_json::from_reader::<File, UserSettings>(config_reader) {
                Ok(x) => x.make_current(),
                Err(_) => {
                    UserSettings::create_default(config)?;
                },
            };
        }
        Ok(())
    }

    pub fn save(config: &Config) -> Result<(), String> {
        let reference = USER_SETTINGS.read().map_err(|_| "Unable to read User Settings")?.clone();

        let json = serde_json::to_string_pretty(&reference).map_err(|_| "Unable to prepare User Settings for saving")?;

        let config_dir = app_config_dir(config).ok_or("Unable to get app_config_dir")?;

        fs::create_dir_all(&config_dir).map_err(|_|"Unable to create directory for User Settings")?;

        fs::write(config_dir.join(USER_SETTINGS_FILENAME), json).map_err(|_| "Unable to save User Settings")?;
        Ok(())
    }

    pub fn current() -> Arc<UserSettings> {
        USER_SETTINGS.read().unwrap().clone()
    }

    pub fn make_current(self) {
        *USER_SETTINGS.write().unwrap() = Arc::new(self)
    }
}

lazy_static::lazy_static! {
    static ref USER_SETTINGS: RwLock<Arc<UserSettings>> = RwLock::new(Default::default());
}
