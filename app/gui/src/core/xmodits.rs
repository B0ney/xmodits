use anyhow::Result;
use std::path::{PathBuf, Path};
use std::time::Duration;
use xmodits_lib::{TrackerModule, Error};
use xmodits_lib::wav::Wav;
use xmodits_lib::load_module;
// use iced::futures::io::
/*
An asynchronous version

*/


#[derive(Default)]
pub struct Ripper {
    modules: Vec<PathBuf>,
    current_module: Option<TrackerModule>,
}

impl Ripper {
    pub fn add_module(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn rip(&mut self, cfg: &crate::gui::SampleConfig) {
        std::thread::sleep(Duration::from_secs(5));

    }
}

pub fn rip_simple(paths: Vec<String>) {
    println!("simple ripping")
}
