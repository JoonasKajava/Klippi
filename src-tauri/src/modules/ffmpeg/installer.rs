use std::fs::{File, self};
use std::io::{Seek, SeekFrom};
use std::path::{PathBuf};
use compress_tools::*;
use anyhow::{Context, bail};
use anyhow::{Result};
use tauri::Window;
use ts_rs::TS;
use crate::modules::file_processing::http::download;

use super::get_version;

const FFMPEG_DOWNLOAD_URL: &str = "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-full.7z";

pub fn uncompress_with_new_name(archive: PathBuf, new_name: &str) -> Result<()> {
    let mut source = File::open(&archive)?;

    let destination = archive.parent().context("Unable to get parent folder of file")?;

    let file_list = list_archive_files(&mut source)?;

    source.seek(SeekFrom::Start(0))?;

    let first_file = file_list.first().context("Unable to get first file from archive")?;

    uncompress_archive(&mut source, &destination, Ownership::Preserve)?;

    drop(source);
    fs::remove_file(&archive)?;

    let uncompressed_folder = destination.join(first_file);
    if uncompressed_folder.is_dir() {
        let full_new_name = uncompressed_folder.with_file_name(new_name);
        println!("Renaming {:?} to {:?}", &uncompressed_folder, &full_new_name);
        fs::rename(&uncompressed_folder, full_new_name)?;
    }

    Ok(())
}

#[derive(Clone, serde::Serialize, TS)]
#[ts(export, export_to="../src/lib/models/")]
pub struct StepChange {
    previous_step: String,
    next_step: String
}

pub async fn install_ffmpeg(window: Window, path: &str, install_location: &PathBuf) -> Result<String>{

    let downloaded_file = download(PathBuf::from(FFMPEG_DOWNLOAD_URL), PathBuf::from(path), |progress| {
        window.emit("download_progress", progress).context("Unable to emit progress").expect("Could not emit event");
    }).await?;

    window.emit("step_change", StepChange {
        previous_step: "Downloading".into(),
        next_step: "Installing".into()

    }).context("Unable to emit").expect("Could not emit event");

    uncompress_with_new_name(downloaded_file, "ffmpeg")?;

    window.emit("step_change", StepChange {
        previous_step: "Installing".into(),
        next_step: "Verifying".into()

    }).context("Unable to emit").expect("Could not emit event");

    let probe_version = get_version("ffprobe", &install_location);
    let ffmpeg_version = get_version("ffmpeg", &install_location);

    if probe_version.is_err() || ffmpeg_version.is_err() {
        bail!("Unable to verify installation")
    }
    window.emit("step_change", StepChange {
        previous_step: "Verifying".into(),
        next_step: "Done".into()

    }).context("Unable to emit").expect("Could not emit event");

    Ok("Successful".into())
}