use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GstreamerBuilderError {
    #[error("Input file has not been specified")]
    InputFileMissing,
    #[error("Gstreamer failed to initialize with error: {0}")]
    GstreamerInitializationFailed(anyhow::Error),
}

pub struct GstreamerBuilder {
    pub input_file: Option<PathBuf>,
}

// Doc: https://github.com/sdroege/gstreamer-rs/blob/main/examples/src/bin/thumbnail.rs
impl GstreamerBuilder {
    pub fn set_input(&mut self, input: impl Into<PathBuf>) {
        // TODO: Check that the file exists?
        self.input_file = Some(input.into());
    }

    pub fn build(self) -> Result<(), GstreamerBuilderError> {
        gstreamer::init()
            .map_err(|e| GstreamerBuilderError::GstreamerInitializationFailed(e.into()))?;

        let binding = &self
            .input_file
            .ok_or(GstreamerBuilderError::InputFileMissing)?;

        let uri = binding
            .to_str()
            .ok_or(GstreamerBuilderError::InputFileMissing)?;

        let pipeline = gstreamer::parse::launch(&format!(
            "uridecodebin uri={uri}
"
        ));

        Ok(())
    }
}
