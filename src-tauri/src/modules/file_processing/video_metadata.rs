use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::{Metadata, self},
    path::{PathBuf},
};

use log::info;
use wax::Glob;

use crate::modules::config::constants::SUPPORTED_VIDEO_EXTENSIONS;



pub fn find_latest_videos(from: &PathBuf, clip_location: &PathBuf) -> Vec<String> {

    let file_matcher = format!("**/*.{{{}}}", SUPPORTED_VIDEO_EXTENSIONS.join(","));
    info!("Searching for videos in: {}", file_matcher);

    let glob = Glob::new(&file_matcher).unwrap();

    let mut result: Vec<String> = Vec::new();
    for entry in glob.walk(from){
        match entry {
            Ok(path) => {
                let path = path.into_path();
                if path.starts_with(clip_location) {
                    continue;
                }
                result.push(path.into_os_string().into_string().unwrap());
            }
            Err(e) => println!("This: {}", e),
        }
    }

    let mut metadata_cache: HashMap<String, fs::Metadata> = HashMap::new();

    result.sort_by(|a, b| {
        let result = compare_file_dates(a, b, &mut metadata_cache);
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
        .or_insert_with(|| fs::metadata(path).expect("Load file metadata"));
    Ok(metadata)
}

fn compare_file_dates<'a>(
    a: &'a String,
    b: &'a String,
    cache: &mut HashMap<String, fs::Metadata>,
) -> Result<Ordering, String> {

    let a_creation_date = {
        let a_meta: &Metadata = get_metadata(a, cache)?;
        a_meta.created().map_err(|_| "Unable to get create date")?
    };

    let b_creation_date = {
        let b_meta: &Metadata = get_metadata(b, cache)?;
        b_meta.created().map_err(|_| "Unable to get create date")?
    };

    Ok(a_creation_date.cmp(&b_creation_date).reverse())
}
