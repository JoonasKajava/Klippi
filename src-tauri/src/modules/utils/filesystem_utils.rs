use std::path::PathBuf;
use md5;


pub trait PathBufExtensions {
    fn to_hashed(&self) -> PathBuf;
}

impl PathBufExtensions for PathBuf {
    fn to_hashed(&self) -> PathBuf {
        let mut new_path = self.clone();

        let digest = md5::compute(self.to_str().unwrap());
        let hash = format!("{:x}", digest);

        new_path.set_file_name(hash);
        return new_path;
    }
}