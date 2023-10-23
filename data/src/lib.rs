//! Data components of XMODITS
//!

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod config;
pub mod history;
pub mod theme;
pub mod error;
pub mod time;

pub use config::Config;
pub use history::History;
pub use theme::Theme;
pub use time::Time;

use xmodits_lib::exporter::AudioFormat;

pub const SUPPORTED_FORMATS: &[AudioFormat] = &[
    AudioFormat::WAV,
    AudioFormat::AIFF,
    AudioFormat::ITS,
    AudioFormat::S3I,
    AudioFormat::IFF,
    AudioFormat::RAW,
];

/// Reimport the xmodits core library.
pub use xmodits_lib;
