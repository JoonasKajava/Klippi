use anyhow::{Context, bail};
use anyhow::{Result};
use futures_util::StreamExt;
use reqwest::{Client};
use ts_rs::TS;
use std::time::{Duration, Instant};
use std::{fs, path::PathBuf};

use std::fs::File;
use std::io::Write;

use std::cmp::min;
#[derive(Clone, serde::Serialize, TS)]
#[ts(export, export_to="../src/lib/models/")]
pub struct DownloadProgress {
    progress: f64,
    total_size: u64,
    downloaded: u64,
    speed: f64,
}

pub async fn get_available_filename(file_path: &PathBuf) -> Result<PathBuf> {
    let mut new_name = file_path.clone();
    let original_name = file_path.file_stem().context("Unable get file stem from file path")?.to_string_lossy();
    let extension = file_path.extension().context("Unable to get file extension")?.to_string_lossy();
    let mut counter = 0;
    while new_name.try_exists()? && counter < 1000 {
        new_name.set_file_name(format!("{original_name} ({counter}).{extension}"));
        counter += 1;
    }

    if new_name.exists() {
        bail!("Tried 1000 times to find suitable file name and failed.")
    }
 
    Ok(new_name)
}

pub async fn download<F : Fn(DownloadProgress)>(
    url: PathBuf,
    target: PathBuf,
    on_progress: F,
) -> Result<PathBuf> {

    let mut parsed_target = target;

    if parsed_target.is_dir() {
        let url_filename = url.file_name().context("Unable to determinate filename since url or target do not contain filename")?;
        parsed_target = parsed_target.join(url_filename);
    }



    println!("Downloading into {}", &parsed_target.display());
    fs::create_dir_all(&parsed_target.parent().context("Unable to folder from filename")?)?;

    let client = Client::new();

    println!("Creating connection to {}", &url.display());

    let res = client
        .get(url.to_string_lossy().to_string())
        .send()
        .await?;

    let total_size = res
        .content_length().context("Could not get content length")?;

    let mut file: std::fs::File;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    let target_file = get_available_filename(&parsed_target).await?;

    let original_extension = &target_file.extension().context("Unable to get file extension")?;
    let temp_filename = &target_file.with_extension(format!("{}.{}", original_extension.to_string_lossy(), "temp"));

    println!("Creating temp file of {}", &target_file.display());
    file = File::create(&temp_filename)?;

    let start = Instant::now();
    println!("Starting download");
    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), total_size);

        downloaded = new;

        let delta: Duration = start.elapsed();
        on_progress(DownloadProgress {
            progress: downloaded as f64 / total_size as f64,
            total_size,
            downloaded,
            speed: downloaded as f64 / delta.as_secs_f64() as f64,
        });
    }

    println!("Renaming {} to {}", &temp_filename.display(), &target_file.display());
    fs::rename(&temp_filename, &target_file)?;

    Ok(target_file)
}
