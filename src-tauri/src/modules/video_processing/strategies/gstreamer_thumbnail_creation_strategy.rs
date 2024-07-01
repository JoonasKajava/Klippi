use std::path::PathBuf;

use gstreamer::prelude::GstBinExt;

use crate::modules::video_processing::thumbnail_creation_strategy::ThumbnailCreationStrategy;
use gstreamer::glib::object::Cast;
use thiserror::Error;

pub struct GstreamerThumbnailCreation {
    pub input_file: Option<PathBuf>,
}

pub enum GstreamerThumbnailCreationSuccess {}

#[derive(Error, Debug)]
pub enum GstreamerThumbnailCreationFailure {
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
pub enum GstreamerThumbnailCreationReport {}

impl ThumbnailCreationStrategy for GstreamerThumbnailCreation {
    type ThumbnailCreationSuccess = GstreamerThumbnailCreationSuccess;

    type ThumbnailCreationFailure = GstreamerThumbnailCreationFailure;

    type ThumbnailCreationReport = GstreamerThumbnailCreationReport;

    fn get_report_channel() -> Option<std::sync::mpsc::Sender<Self::ThumbnailCreationReport>> {
        None
    }

    fn execute(self) -> Result<Self::ThumbnailCreationSuccess, Self::ThumbnailCreationFailure> {
        gstreamer::init().map_err(|e| {
            GstreamerThumbnailCreationFailure::GstreamerInitializationFailed(e.into())
        })?;

        let pipeline = self.create_pipeline()?;

        let appsink = pipeline;

        Ok(())
    }
}

impl GstreamerThumbnailCreation {
    pub fn set_input(&mut self, input: impl Into<PathBuf>) {
        // TODO: Check that the file exists?
        self.input_file = Some(input.into());
    }

    fn get_input(&self) -> Option<&str> {
        self.input_file.clone()?.to_str()
    }

    fn create_pipeline(&self) -> Result<gstreamer::Pipeline, GstreamerThumbnailCreationFailure> {
        let uri = self
            .get_input()
            .ok_or(GstreamerThumbnailCreationFailure::InputFileMissing)?;

        gstreamer::parse::launch(&format!(
            "uridecodebin uri={uri} ! videoconvert ! appsink name=sink
"
        ))
        .map_err(|e| GstreamerThumbnailCreationFailure::FailedToCreatePipeline(e))?
        .downcast::<gstreamer::Pipeline>()
        .map_err(GstreamerThumbnailCreationFailure::FailedToDowncast)
    }

    fn create_appsink(
        &self,
        pipeline: &gstreamer::Pipeline,
    ) -> Result<gstreamer_app::AppSink, GstreamerThumbnailCreationFailure> {
        pipeline
            .by_name("sink")
            .ok_or(GstreamerThumbnailCreationFailure::AppsinkError(
                "Failed to get app sink from pipeline".to_string(),
            ))?
            .downcast::<gstreamer_app::AppSink>()
            .map_err(GstreamerThumbnailCreationFailure::FailedToDowncast)
    }
}
