use std::sync::mpsc::Sender;

pub trait ThumbnailCreationStrategy {
    type ThumbnailCreationSuccess;
    type ThumbnailCreationFailure;
    type ThumbnailCreationReport;

    fn get_report_channel() -> Option<Sender<Self::ThumbnailCreationReport>>;

    fn execute(self) -> Result<Self::ThumbnailCreationSuccess, Self::ThumbnailCreationFailure>;
}
