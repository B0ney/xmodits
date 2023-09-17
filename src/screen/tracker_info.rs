//! Display information about a tracker module
//!
//! Also allows the user to play the different samples.

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use data::xmodits_lib::Module;
use iced::Element;

#[derive(Debug, Clone)]
pub enum TrackerInfo {
    Invalid {
        path: PathBuf,
        reason: String,
    },
    Loaded {
        path: PathBuf,
        name: String,
        format: String,
        samples: usize,
        total_sample_size: usize,
    },
}

impl TrackerInfo {
    pub fn path(&self) -> &Path {
        match self {
            Self::Invalid { path, .. } 
            | Self::Loaded { path, .. } => path,
        }
    }

    pub fn matches_path(&self, other: &Path) -> bool {
        self.path() == other
    }

    pub fn invalid(error: String, path: PathBuf) -> Self {
        Self::Invalid {
            reason: error,
            path,
        }
    }

    pub fn valid(tracker: Box<dyn Module>, path: PathBuf) -> Self {
        Self::Loaded {
            name: tracker.name().to_owned(),
            format: tracker.format().to_owned(),
            samples: tracker.total_samples(),
            path,
            total_sample_size: tracker
                .samples()
                .iter()
                .map(|f| f.length as usize)
                .sum::<usize>()
                / 1024,
        }
    }
}

pub fn view() {}

pub async fn probe(path: PathBuf) -> TrackerInfo {
    todo!()
}