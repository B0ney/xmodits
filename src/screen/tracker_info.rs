//! Display information about a tracker module
//!
//! Also allows the user to play the different samples.

use std::path::{Path, PathBuf};

use crate::app::Message;
use crate::utils::filename;
use crate::widget::helpers::{centered_container, control_filled};
use crate::widget::{Collection, Element};

use iced::widget::{button, column, text, Space};
use iced::Alignment;
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
            TrackerInfo::Invalid { path, .. } | TrackerInfo::Loaded { path, .. } => path == other.as_ref(),
        }
    }

    pub fn invalid(error: String, path: PathBuf) -> Self {
        Self::Invalid { reason: error, path }
    }

    pub fn clear(&mut self) {
        *self = Self::None;
    }

    pub fn view(&self) -> Element<Message> {
        let title = "Current Tracker Information";

        let content = match &self {
            TrackerInfo::None => column![text("None Selected")],
            TrackerInfo::Invalid { path, reason } => {
                let error = format!("Failed to load {}", filename(path));
                column![text(error), text(reason)]
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
                let view_samples_button = None::<Element<Message>>;

                column![
                    text(format!("Module Name: {}", name.trim())),
                    text(format!("Format: {}", format)),
                    text(format!("Samples: {}", samples)),
                    text(format!("Total Sample Size: {} KiB", total_sample_size)),
                ]
                .push_maybe(view_samples_button.map(|btn| column![Space::with_width(15), btn]))
            }
        };

        let content = centered_container(content.align_items(Alignment::Center).spacing(5)).padding(8);

        control_filled(title, content).into()
    }
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
