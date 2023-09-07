pub use super::SampleNameConfig;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use xmodits_lib::exporter::AudioFormat;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SampleRippingConfig {
    pub destination: PathBuf,
    pub self_contained: bool,
    pub folder_max_depth: u8,
    pub strict: bool,
    pub worker_threads: usize,
    pub exported_format: AudioFormat,
    // must be placed at the bottom
    pub naming: SampleNameConfig,
}

impl Default for SampleRippingConfig {
    fn default() -> Self {
        Self {
            destination: dirs::download_dir().expect("Expected Downloads folder"),
            self_contained: true,
            folder_max_depth: 4,
            strict: true,
            exported_format: Default::default(),
            naming: SampleNameConfig {
                index_padding: 2,
                prefer_filename: true,
                ..Default::default()
            },
            worker_threads: 0,
        }
    }
}
