use std::path::PathBuf;

// TODO: improve imports
use xmodits_lib::interface::errors::{Error, ExtractionError, FailedExtraction};

pub struct Extraction {
    path: PathBuf,
    reason: Reason,
}

pub enum Reason {
    /// A module could not be ripped due to a single reason
    Single(),
    /// Some or none of the samples could not be ripped
    Multiple,
}
