use serde::{Deserialize, Serialize};
use xmodits_lib::{SampleNamer, SampleNamerTrait};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SampleNameConfig {
    pub index_raw: bool,
    pub index_only: bool,
    pub index_padding: u8,
    pub upper: bool,
    pub lower: bool,
    pub prefix: bool,
    pub prefer_filename: bool,
}

impl Default for SampleNameConfig {
    fn default() -> Self {
        Self {
            index_raw: false,
            index_only: false,
            index_padding: 2,
            upper: false,
            lower: false,
            prefix: false,
            prefer_filename: true,
        }
    }
}

impl SampleNameConfig {
    pub fn build_func(&self) -> Box<dyn SampleNamerTrait> {
        SampleNamer {
            index_only: self.index_only,
            index_padding: self.index_padding,
            index_raw: self.index_raw,
            lower: self.lower,
            upper: self.upper,
            prefix_source: self.prefix,
            prefer_filename: self.prefer_filename,
            ..Default::default()
        }
        .into()
    }
}
