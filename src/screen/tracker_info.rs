//! Display information about a tracker module
//!
//! Also allows the user to play the different samples.

use std::path::{Path, PathBuf};

use crate::app::Message;
use crate::utils::filename;
use crate::widget::helpers::centered_container;
use crate::widget::helpers::{centered_column_x, centered_text, control_filled};
use crate::widget::{Button, Collection, Element};

use iced::widget::{button, column, text, Space};
use xmodits_lib::common::info::Info;

#[derive(Default, Debug, Clone)]
pub enum TrackerInfo {
    #[default]
    None,
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
    pub fn matches_path(&self, other: impl AsRef<Path>) -> bool {
        match self {
            TrackerInfo::None => false,
            TrackerInfo::Invalid { path, .. } | 
            TrackerInfo::Loaded { path, .. } => path == other.as_ref(),
        }
    }

    pub fn invalid(error: String, path: PathBuf) -> Self {
        Self::Invalid { reason: error, path }
    }

    pub fn clear(&mut self) {
        *self = Self::None;
    }
}

pub fn view(tracker_info: &TrackerInfo) -> Element<Message> {
    let title = "Current Tracker Information";

    let content = match tracker_info {
        TrackerInfo::None => column![centered_text("None Selected")],
        TrackerInfo::Invalid { path, reason } => {
            column![
                centered_text(format!("Failed to load {}", filename(path))),
                centered_text(reason),
            ]
        }
        
        TrackerInfo::Loaded {
            path,
            name,
            format,
            samples,
            total_sample_size,
        } => {
            #[cfg(feature = "audio")]
            let view_samples_button = Some(
                button("View Samples")
                    .on_press(Message::PreviewSamples(path.to_owned()))
                    .padding(5),
            );

            #[cfg(not(feature = "audio"))]
            let view_samples_button: Option<Button<Message>> = None;

            column![
                centered_text(format!("Module Name: {}", name.trim())),
                centered_text(format!("Format: {}", format)),
                centered_text(format!("Samples: {}", samples)),
                centered_text(format!("Total Sample Size: {} KiB", total_sample_size)),
            ]
            .push_maybe(view_samples_button.map(|btn| column![Space::with_width(15), btn]))
        }
    };

    let content = centered_container(centered_column_x(content)).padding(8);

    control_filled(title, content).into()
}

pub async fn probe(path: PathBuf) -> TrackerInfo {
    tokio::task::spawn_blocking(move || match Info::new(&path) {
        Ok(Info {
            name,
            format,
            total_samples,
            total_sample_size,
        }) => TrackerInfo::Loaded {
            path,
            name,
            format,
            samples: total_samples,
            total_sample_size,
        },
        Err(reason) => TrackerInfo::Invalid {
            path,
            reason: reason.to_string(),
        },
    })
    .await
    .unwrap()
}
