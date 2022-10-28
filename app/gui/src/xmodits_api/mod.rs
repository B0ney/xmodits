use anyhow::Result;
use std::path::PathBuf;
use xmodits_lib::TrackerModule;

#[derive(Default)]
pub struct Ripper {
    modules: Vec<PathBuf>,
    current_module: Option<TrackerModule>,
}

impl Ripper {
    pub fn add_module(&mut self) -> Result<()> {
        Ok(())
    }
}

pub fn rip_simple(paths: Vec<String>) {
    println!("simple ripping")
}
