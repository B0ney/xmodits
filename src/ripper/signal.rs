use std::path::PathBuf;

use super::filter::CustomFilter;
use data::config;

/// Constructed and sent by the main GUI
/// to the subscription
pub struct Signal {
    pub entries: Vec<PathBuf>,
    pub ripping: config::SampleRippingConfig,
    pub naming: config::SampleNameConfig,
    // pub filter: Option<Box<dyn CustomFilter>>,
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
            // filter: None,
            entries,
        }
    }

    // pub fn with_filter(mut self, filter: Box<dyn CustomFilter>) -> Self {
    //     self.filter = Some(filter);
    //     self
    // }
}
