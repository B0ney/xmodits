//! Data components of XMODITS

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

#[cfg(feature = "manual")]
pub static MANUAL: &str = include_str!("../../assets/manual.txt");