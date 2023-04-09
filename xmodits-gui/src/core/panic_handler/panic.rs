use std::path::PathBuf;
use crate::core::track::GLOBAL_TRACKER;

pub enum Source {
    XmoditsLIB,
    XmoditsGUI,
    Other(String),
}

pub struct Crash {
    source: Source,
    bad_module: Option<BadModule>,
    location: String,
    line: u32,
}

pub enum BadModule {
    Exact(PathBuf),
    Suspects {
        traversed_entries: PathBuf,
        offset: u64,
        window: u64,
    }
}