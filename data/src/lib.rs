//! Data components of XMODITS
//! 

pub mod config;
pub mod history;
pub mod theme;
// pub mod entries;
pub mod error;
pub mod time;
pub mod tracker_info;
pub mod name_preview;

pub use theme::Theme;
pub use config::Config;


use xmodits_lib::exporter::AudioFormat;

pub const SUPPORTED_FORMATS: &[AudioFormat] = &[
    AudioFormat::WAV,
    AudioFormat::AIFF,
    AudioFormat::ITS,
    AudioFormat::S3I,
    AudioFormat::IFF,
    AudioFormat::RAW,
];

pub use xmodits_lib;