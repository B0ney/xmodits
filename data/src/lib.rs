//! Data components of XMODITS
//!

pub mod config;
pub mod history;
pub mod theme;
// pub mod entries;
pub mod error;
pub mod name_preview;
pub mod time;

pub use history::History;
pub use config::Config;
pub use theme::Theme;

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
