//! Configure how samples should be extracted

use std::path::PathBuf;

use data::config::SampleRippingConfig;
use data::xmodits_lib::exporter::AudioFormat;

use crate::widget::Element;
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, pick_list, row, text, text_input,
};
use iced::Command;

use once_cell::sync::Lazy;

use crate::utils::folder_dialog;

// TODO have data defined here

#[derive(Debug, Clone)]
pub enum Message {
    SetExportFormat(AudioFormat),
    ToggleSelfContained(bool),
    ToggleStrictLoad(bool),
    SetWorkerThreads(Workers),
    SetFolderDepth(u8),
    SetDestination(Option<PathBuf>),
    SetDestinationDialog,
}

pub fn update(cfg: &mut SampleRippingConfig, message: Message) -> Command<Message> {
    match message {
        Message::SetExportFormat(format) => cfg.exported_format = format,
        Message::ToggleSelfContained(toggle) => cfg.self_contained = toggle,
        Message::SetFolderDepth(depth) => cfg.folder_max_depth = depth,
        Message::ToggleStrictLoad(strict) => cfg.strict = strict,
        Message::SetWorkerThreads(Workers(threads)) => cfg.worker_threads = threads,
        Message::SetDestination(destination) => {
            if let Some(destination) = destination {
                cfg.destination = destination
            }
        }
        Message::SetDestinationDialog => {
            return Command::perform(folder_dialog(), Message::SetDestination);
        }
    }
    Command::none()
}

pub static DESTINATION_BAR_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub fn view_destination_bar(destination: &str) -> Element<Message> {
    let input = text_input("Output Directory", destination)
        .id(DESTINATION_BAR_ID.clone())
        .on_input(|f| {
            let destination = PathBuf::new().join(f);
            Message::SetDestination(Some(destination))
        });

    let button = button("Select")
        .on_press(Message::SetDestinationDialog)
        .padding(10);

    row![input, button].into()
}

pub fn view<'a>(ripping: &'a SampleRippingConfig) -> Element<'a, Message> {
    let col1 = column![
        checkbox(
            "Self Contained",
            ripping.self_contained,
            Message::ToggleSelfContained
        ),
        checkbox("Strict Loading", ripping.strict, Message::ToggleStrictLoad),
    ];

    let export_format = row![
        pick_list(
            data::SUPPORTED_FORMATS,
            Some(ripping.exported_format),
            Message::SetExportFormat
        ),
        text("Export Format"),
    ];

    let options: &[u8] = &[1, 2, 3, 4, 5, 6, 7];
    let folder_scan_depth = row![
        pick_list(
            options,
            Some(ripping.folder_max_depth),
            Message::SetFolderDepth
        ),
        text("Export Format"),
    ];

    let options = [0usize, 1, 2, 4, 6, 8, 10, 12, 16].map(Workers).to_vec();
    let worker_threads = row![
        pick_list(
            options,
            Some(Workers(ripping.worker_threads)),
            Message::SetWorkerThreads
        ),
        text("Worker Threads"),
    ];

    let settings = column![
        col1,
        export_format,
        worker_threads,
        horizontal_rule(1),
        folder_scan_depth
    ];

    let settings = column![text("Ripping Configuration"), settings];

    container(settings).into()
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
#[repr(transparent)]
pub struct Workers(pub usize);

impl std::fmt::Display for Workers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "Automatic"),
            n => write!(f, "{}", n),
        }
    }
}
