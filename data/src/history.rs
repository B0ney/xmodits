//! provide detailed informatiomn about the modules ripped

use std::path::PathBuf;

use crate::entries::Entries;
use crate::config::SampleRippingConfig;

#[derive(Default)]
pub struct History {
    timestamp: chrono::DateTime<chrono::Utc>,
    entries: Entries,
    // failed: Option<Failed>,
    sample_config: SampleRippingConfig,
}