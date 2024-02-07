use data::config;
use std::path::PathBuf;

/// Constructed and sent by the main GUI
/// to the subscription
#[derive(Debug)]
pub struct Signal {
    pub entries: Vec<PathBuf>,
    pub ripping: config::SampleRippingConfig,
    pub naming: config::SampleNameConfig,
}

impl Signal {
    pub fn new(
        entries: Vec<PathBuf>,
        ripping: config::SampleRippingConfig,
        naming: config::SampleNameConfig,
    ) -> Self {
        Self {
            ripping,
            naming,
            entries,
        }
    }
}
