//! Display information about a tracker module
//!
//! Also allows the user to play the different samples.

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use crate::{
    utils::filename,
    widget::{
        helpers::{centered_column, centered_text},
        Element,
    },
};
use data::xmodits_lib::Module;
use iced::{
    widget::{button, column, container, row, text, Space},
    Length,
};
// pub enum Message {
//     PreviewSample,
// }

use crate::{
    app::Message,
    widget::helpers::{centered_container, control},
};

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
            Self::Invalid { path, .. } | Self::Loaded { path, .. } => path,
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

pub fn view<'a>(tracker_info: Option<&TrackerInfo>) -> Element<'a, Message> {
    let Some(info) = tracker_info else {
        return control(
            "Current Tracker Information",
            centered_container(text("None Selected")),
        )
        .into();
    };

    let content = match info {
        TrackerInfo::Invalid { path, reason } => centered_column(column![
            centered_text(format!("Failed to load {}", filename(path))),
            centered_text(reason),
        ]),
        TrackerInfo::Loaded {
            path: _,
            name,
            format,
            samples,
            total_sample_size,
        } => {
            let view_samples_button = button("View Samples").on_press(Message::Ignore).padding(5);

            centered_column(column![
                text(format!("Module Name: {}", name.trim())),
                text(format!("Format: {}", format)),
                text(format!("Samples: {}", samples)),
                text(format!("Total Sample Size: {} KiB", total_sample_size)),
                Space::with_width(15),
                view_samples_button,
            ])
        }
    };

    control("Current Tracker Information", content).into()
}

pub async fn probe(path: PathBuf) -> TrackerInfo {
    todo!()
}
