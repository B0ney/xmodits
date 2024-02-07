use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct SampleNameParams {
    pub module_name: String,
    pub module_source: PathBuf,
    pub sample_filename: Option<String>,
    pub sample_name: String,
    pub raw_index: u16,
    pub seq_index: u16,
}

impl Default for SampleNameParams {
    fn default() -> Self {
        Self {
            module_name: String::from("music"),
            sample_filename: Some(String::from("Kick_1.wav")),
            sample_name: String::from("Kick.wav"),
            module_source: PathBuf::from("~/Downloads/music.it"),
            raw_index: 7,
            seq_index: 0,
        }
    }
}
