use log::error;
use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};
use wax::Glob;

use super::config::constants::{DAYS_TO_KEEP_THUMBNAILS, THUMBNAIL_EXTENSION};

pub fn is_eligible_for_cleanup(path: impl AsRef<Path>) -> bool {
    let metadata = match fs::metadata(&path) {
        Ok(m) => m,
        Err(_) => {
            error!("Error reading metadata for path: {:?}", path.as_ref());
            return false;
        }
    };
    if !metadata.is_file() {
        error!("Path is not a file: {:?}", path.as_ref());
        return false;
    }

    let now = std::time::SystemTime::now();
    match metadata.modified() {
        Ok(c) => now - Duration::from_secs(DAYS_TO_KEEP_THUMBNAILS * 24 * 60 * 60) > c,
        Err(_) => false,
    }
}

pub fn clean_thumbnails(thumbnails_path: impl AsRef<Path>) {
    let glob_pattern = &format!("**/*.{}", THUMBNAIL_EXTENSION);

    let glob = Glob::new(glob_pattern).unwrap();

    if !thumbnails_path.as_ref().exists() {
        if let Err(e) = fs::create_dir_all(&thumbnails_path) {
            error!(
                "Error creating thumbnails directory {:?}: {}",
                thumbnails_path.as_ref(),
                e
            );
        }
    }

    let result = glob
        .walk(thumbnails_path)
        .map(|x| match x {
            Ok(x) => Some(x),
            Err(e) => {
                error!("Error reading thumbnail path: {}", e);
                None
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap().into_path())
        .collect::<Vec<PathBuf>>();

    for path in result {
        if !is_eligible_for_cleanup(&path) {
            continue;
        }
        match std::fs::remove_file(path) {
            Ok(_) => {}
            Err(e) => error!("Error deleting thumbnail: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};
    use tempfile::tempdir;

    use filetime_creation::{set_file_ctime, set_file_mtime, FileTime};
    use std::fs::File;

    use super::*;

    #[test]
    fn test_is_eligible_for_cleanup() -> Result<()> {
        assert_eq!(
            is_eligible_for_cleanup(&PathBuf::new()),
            false,
            "Empty path should not be eligible for cleanup"
        );

        let temp_dir = tempdir()?;

        assert_eq!(
            is_eligible_for_cleanup(&temp_dir),
            false,
            "Empty directory should not be eligible for cleanup"
        );

        let new_file = create_file(&temp_dir, format!("new_file.{}", THUMBNAIL_EXTENSION), 0)?;

        assert_eq!(
            is_eligible_for_cleanup(&new_file),
            false,
            "File should not be eligible for cleanup if it was modified less than {} days ago",
            DAYS_TO_KEEP_THUMBNAILS
        );

        let old_file = create_file(
            &temp_dir,
            format!("old_file.{}", THUMBNAIL_EXTENSION),
            DAYS_TO_KEEP_THUMBNAILS + 1,
        )?;

        assert_eq!(
            is_eligible_for_cleanup(&old_file),
            true,
            "File should be eligible for cleanup if modified it was more than {} days ago",
            DAYS_TO_KEEP_THUMBNAILS
        );

        Ok(())
    }

    fn create_file(
        path: impl AsRef<Path>,
        name: impl Into<String>,
        age_in_days: u64,
    ) -> Result<PathBuf> {
        let new_file = &path.as_ref().join(name.into());
        File::create(&new_file)?;

        let new_time = FileTime::from_system_time(
            std::time::SystemTime::now() - Duration::from_secs(age_in_days * 24 * 60 * 60),
        );
        set_file_mtime(&new_file, new_time)?;
        set_file_ctime(&new_file, new_time)?;

        Ok(new_file.to_path_buf())
    }

    #[test]
    fn test_clean_thumbnails() -> Result<()> {
        let temp_dir = tempdir()?;

        let new_file = create_file(&temp_dir, format!("new_file.{}", THUMBNAIL_EXTENSION), 0)?;
        let old_file = create_file(
            &temp_dir,
            format!("old_file.{}", THUMBNAIL_EXTENSION),
            DAYS_TO_KEEP_THUMBNAILS + 1,
        )?;
        let new_non_thumbnail_file = create_file(&temp_dir, "new_non_thumbnail_file.pdf", 0)?;
        let old_non_thumbnail_file = create_file(
            &temp_dir,
            "old_non_thumbnail_file.pdf",
            DAYS_TO_KEEP_THUMBNAILS + 1,
        )?;

        clean_thumbnails(&temp_dir);

        assert_eq!(new_file.exists(), true, "New file should not be deleted");
        assert_eq!(old_file.exists(), false, "Old file should be deleted");
        assert_eq!(
            new_non_thumbnail_file.exists(),
            true,
            "New non-thumbnail file should not be deleted"
        );
        assert_eq!(
            old_non_thumbnail_file.exists(),
            true,
            "Old non-thumbnail file should not be deleted"
        );

        Ok(())
    }
}
