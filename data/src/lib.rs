//! Data components of XMODITS
//!

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod config;
pub mod theme;
pub mod time;

pub use config::Config;
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
