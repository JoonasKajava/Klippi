pub mod app_config;
pub mod user_settings;

pub mod constants;

use log::error;

#[derive(Debug)]
pub struct Configuration {
    pub app_config: Mutex<AppConfig>,
    pub user_settings: Mutex<UserSettings>,
}

impl Configuration {
    pub fn init(app: &AppHandle) -> Self {
        let config_dir = app.path().app_config_dir().expect("Unable to get config dir");
        Configuration {
            app_config: Mutex::new(AppConfig::init(
                config_dir.join("app_config.toml"),
                AppConfig::default(app),
            )),
            user_settings: Mutex::new(UserSettings::init(
                config_dir.join("user_settings.toml"),
                UserSettings::default(app),
            )),
        }
    }
}

use std::{
    fs::{self},
    path::Path,
    sync::Mutex,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use self::{app_config::AppConfig, user_settings::UserSettings};

pub trait Init {
    fn init(config_location: impl AsRef<Path>, default: Self) -> Self;
}

pub trait Save {
    fn save(&self, config_location: impl AsRef<Path>) -> Result<()>;
}

pub trait DefaultValues {
    fn default(app: &AppHandle) -> Self;
}

pub trait FileConfig: Save + Init + DefaultValues {}

impl<T: FileConfig + for<'de> Deserialize<'de>> Init for T {
    fn init(config_location: impl AsRef<Path>, default: Self) -> Self {
        if !config_location.as_ref().try_exists().unwrap_or(false) {
            let _ = &default
                .save(&config_location)
                .expect("Unable to save config");
            default
        } else {
            let config_string =
                fs::read_to_string(config_location).expect("Unable to open config file");
            match toml::from_str::<Self>(&config_string) {
                Ok(x) => x,
                Err(e) => {
                    println!("{:?}", e);
                    error!("Unable to deserialize config, using default: {:?}", e);
                    default
                }
            }
        }
    }
}

impl<T: FileConfig + Serialize> Save for T {
    fn save(&self, config_location: impl AsRef<Path>) -> anyhow::Result<()> {
        let toml = toml::to_string_pretty(&self)?;

        let config_dir = config_location
            .as_ref()
            .parent()
            .context("Unable to get config location")?;

        fs::create_dir_all(config_dir)?;

        fs::write(config_location, toml)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Write};

    use super::*;
    use anyhow::Ok;
    use tempfile::tempdir;
    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct TestConfig {
        pub field1: String,
        pub field2: String,
    }

    impl Default for TestConfig {
        fn default() -> Self {
            TestConfig {
                field1: "default_field1".into(),
                field2: "default_field2".into(),
            }
        }
    }

    impl DefaultValues for TestConfig {
        fn default(_app: &AppHandle) -> Self {
            TestConfig {
                field1: "default_field1".into(),
                field2: "default_field2".into(),
            }
        }
    }

    impl FileConfig for TestConfig {}

    #[test]
    pub fn test_init_creates_file() -> Result<()> {
        let temp_dir = tempdir()?;

        let test_config_location = temp_dir.path().join("test_config.toml");

        let _ = TestConfig::init(&test_config_location, Default::default());

        assert_eq!(
            test_config_location.exists(),
            true,
            "Config file initialization must create a file if it doesn't exist"
        );

        Ok(())
    }

    #[test]
    pub fn test_init_self_correction() -> Result<()> {
        let temp_dir = tempdir()?;

        let test_config_location = temp_dir.path().join("test_config.toml");

        {
            #[derive(Serialize)]
            struct InvalidConfig {
                pub field1: String,
                pub field2: i64,
            }

            let mut file = File::create(&test_config_location)?;
            let toml = toml::to_string_pretty(&InvalidConfig {
                field1: "existing_field1".into(),
                field2: 123,
            })?;
            file.write_all(toml.as_bytes())?;
        }

        let existing_config = TestConfig::init(&test_config_location, Default::default());

        assert_eq!(
            existing_config.field1, "default_field1",
            "Config file initialization must replace invalid values with default values"
        );

        assert_eq!(
            existing_config.field2, "default_field2",
            "Config file initialization must replace invalid values with default values"
        );

        Ok(())
    }

    #[test]
    pub fn test_init_loads_existing_config() -> Result<()> {
        let temp_dir = tempdir()?;

        let test_config_location = temp_dir.path().join("test_config.toml");

        {
            let mut file = File::create(&test_config_location)?;
            let toml = toml::to_string_pretty(&TestConfig {
                field1: "existing_field1".into(),
                field2: "existing_field2".into(),
            })?;
            file.write_all(toml.as_bytes())?;
        }

        let existing_config = TestConfig::init(&test_config_location, Default::default());

        assert_eq!(
            existing_config.field1, "existing_field1",
            "Config file initialization must load existing config"
        );
        assert_eq!(
            existing_config.field2, "existing_field2",
            "Config file initialization must load existing config"
        );
        Ok(())
    }
}
