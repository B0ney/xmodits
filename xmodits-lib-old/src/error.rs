/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

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
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    FileError(String),

    #[error("Module has no samples")]
    EmptyModule,

    #[error("Generic Error: {0}")]
    GenericError(String),

    #[error("Multiple Errors")]
    MultipleErrors(Vec<XmoditsError>),

    #[error("This module has caused an out of bounds error. This is a bug. Please add this module to your bug report.")]
    OutOfBounds
}

impl XmoditsError {
    pub fn invalid(e: &str) -> Self {
        Self::InvalidModule(e.to_owned())
    }

    pub fn unsupported(e: &str) -> Self {
        Self::UnsupportedFormat(e.to_owned())
    }

    pub fn file(e: &str) -> Self {
        Self::FileError(e.to_owned())
    }
    pub fn out_of_bounds() -> Self {
        Self::OutOfBounds
    }
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
