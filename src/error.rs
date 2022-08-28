use thiserror::Error;

#[derive(Error, Debug)]
pub enum XmoditsError {
    #[error("Failed to rip sample: {0} ")]
    SampleExtractionFailure(String),

    #[error("{0}")]
    UnsupportedFormat(String),

    #[error("Invalid Module: {0}")]
    InvalidModule(String),

    #[error("{0}")]
    IoError(#[from]std::io::Error),

    #[error("{0}")]
    FileError(String),

    #[error("Module has no samples")]
    EmptyModule,

    #[error("Generic Error: {0}")]
    GenericError(String)
}

impl From<&str> for XmoditsError {
    fn from(e: &str) -> Self {
        Self::GenericError(e.to_owned())
    }
}
impl From<String> for XmoditsError {
    fn from(e: String) -> Self {
        Self::GenericError(e)
    }
}