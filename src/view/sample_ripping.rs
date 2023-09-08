//! Configure how samples should be extracted

use data::config::SampleRippingConfig;
// use data::xmodits_lib::AudioFormat;
use data::xmodits_lib::exporter::AudioFormat;

use iced::widget::{checkbox, column, container, horizontal_rule, pick_list, row, text};
use iced::Element;

#[derive(Debug, Clone)]
pub enum Message {
    SetExportFormat(AudioFormat),
    ToggleSelfContained(bool),
    ToggleStrictLoad(bool),
    SetWorkerThreads(Workers),
    SetFolderDepth(u8),
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

pub fn update(cfg: &mut SampleRippingConfig, message: Message) {
    match message {
        Message::SetExportFormat(format) => cfg.exported_format = format,
        Message::ToggleSelfContained(toggle) => cfg.self_contained = toggle,
        Message::SetFolderDepth(depth) => cfg.folder_max_depth = depth,
        Message::ToggleStrictLoad(strict) => cfg.strict = strict,
        Message::SetWorkerThreads(Workers(threads)) => cfg.worker_threads = threads,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
#[repr(transparent)]
pub struct Workers(pub usize);

impl std::fmt::Display for Workers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "Automatic"),
            n => write!(f, "{}", format!("{}", n)),
        }
    }
}
