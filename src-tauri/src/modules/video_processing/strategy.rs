pub trait CancelableStrategy {
    type CancelSuccess;
    type CancelError;
    fn cancel() -> Result<Self::CancelSuccess, Self::CancelError>;
}
