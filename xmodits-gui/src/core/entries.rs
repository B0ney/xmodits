use std::{fs::File, path::PathBuf};

use chrono::Utc;
use xmodits_lib::interface::Error;

use super::cfg::SampleRippingConfig;

pub struct Entries {
    all_selected: bool,
    entries: Vec<Entry>,
}

pub struct Entry {
    selected: bool,
    path: PathBuf,
}

struct History {
    timestamp: chrono::DateTime<Utc>,
    entries: Entries,
    failed: Option<Failed>,
    config: SampleRippingConfig,
}

enum Failed {
    Mem(Vec<FailedModule>),
    File { path: PathBuf, file: File },
}

struct FailedModule {
    path: PathBuf,
    reason: String,
}
