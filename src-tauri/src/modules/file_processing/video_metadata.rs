use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::{Metadata, self},
    path::{Path, PathBuf},
};

extern crate ffmpeg_next as ffmpeg;

use glob::glob;
use anyhow::{anyhow, Error};
use crate::modules::config::user_settings::UserSettings;

pub fn get_thumbnail(file: &PathBuf) -> Result<PathBuf, Error> {
    if !file.exists() {
        return Err(anyhow!(format!("Cannot get thumbnail from a file that does not exist {}", file.display())));
    }
    ffmpeg::init()?;

    let input =  ffmpeg::format::input(file)?;
    println!("Duration {}", input.duration());

    Ok(PathBuf::new())
}

pub fn find_lastest_videos() -> Vec<String> {
    let settings = UserSettings::current();
    let regex = Path::new(&settings.videos_directory)
        .join("**/*.mp4")
        .into_os_string()
        .into_string()
        .unwrap();
    let mut result: Vec<String> = Vec::new();
    for entry in glob(regex.as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.starts_with(&settings.clip_location) {
                    continue;
                }
                result.push(path.into_os_string().into_string().unwrap());
            }
            Err(e) => println!("This: {}", e),
        }
    }

    let mut metadata_cache: HashMap<String, fs::Metadata> = HashMap::new();

    result.sort_by(|a, b| {
        let result = compare_file_dates(&a, &b, &mut metadata_cache);
        match result {
            Ok(ord) => ord,
            Err(e) => {println!("{}", e); Ordering::Less},
        }
    });
    result
}

fn get_metadata<'a>(
    path: &String,
    cache: &'a mut HashMap<String, fs::Metadata>,
) -> Result<&'a fs::Metadata, String> {
    let metadata = cache
        .entry(path.clone())
        .or_insert_with(|| fs::metadata(&path).expect("Load file metadata"));
    Ok(metadata)
}

fn compare_file_dates<'a>(
    a: &'a String,
    b: &'a String,
    cache: &mut HashMap<String, fs::Metadata>,
) -> Result<Ordering, String> {

    let a_creation_date = {
        let a_meta: &Metadata = get_metadata(&a, cache)?;
        a_meta.created().map_err(|_| "Unable to get create date")?
    };

    let b_creation_date = {
        let b_meta: &Metadata = get_metadata(&b, cache)?;
        b_meta.created().map_err(|_| "Unable to get create date")?
    };

    Ok(a_creation_date.cmp(&b_creation_date).reverse())
}
