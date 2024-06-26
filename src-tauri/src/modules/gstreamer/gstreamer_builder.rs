use gstreamer::prelude::GstBinExt;
use std::path::PathBuf;

use gstreamer::{glib::object::Cast, traits::GstBinExt};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GstreamerBuilderError {
    #[error("Input file has not been specified")]
    InputFileMissing,
    #[error("Gstreamer failed to initialize with error: {0}")]
    GstreamerInitializationFailed(anyhow::Error),
    #[error("Gstreamer failed to create pipeline")]
    FailedToCreatePipeline(gstreamer::glib::Error),
    #[error("Gstreamer failed to downcast element")]
    FailedToDowncast,
    #[error("Encountered error with appsink: {0}")]
    AppsinkError(String),
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

    fn get_input(&self) -> Option<&str> {
        self.input_file.clone()?.to_str()
    }

    fn create_pipeline(&self) -> Result<gstreamer::Pipeline, GstreamerBuilderError> {
        let uri = self
            .get_input()
            .ok_or(GstreamerBuilderError::InputFileMissing)?;

        gstreamer::parse::launch(&format!(
            "uridecodebin uri={uri} ! videoconvert ! appsink name=sink
"
        ))
        .map_err(|e| GstreamerBuilderError::FailedToCreatePipeline(e))?
        .downcast::<gstreamer::Pipeline>()
        .map_err(GstreamerBuilderError::FailedToDowncast)
    }

    fn create_appsink(
        &self,
        pipeline: &gstreamer::Pipeline,
    ) -> Result<gstreamer_app::AppSink, GstreamerBuilderError> {
        pipeline
            .by_name("sink")
            .ok_or(GstreamerBuilderError::AppsinkError(
                "Failed to get app sink from pipeline".to_string(),
            ))?
            .downcast::<gstreamer_app::AppSink>()
            .map_err(GstreamerBuilderError::FailedToDowncast)
    }

    pub fn build(self) -> Result<(), GstreamerBuilderError> {
        gstreamer::init()
            .map_err(|e| GstreamerBuilderError::GstreamerInitializationFailed(e.into()))?;

        let pipeline = self.create_pipeline()?;

        let appsink = pipeline;

        Ok(())
    }
}
