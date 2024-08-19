use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use xmodits_lib::export::Format;

use crate::style;
use crate::widget::helpers::{centered_button, centered_column_x, control, labelled_picklist};
use crate::widget::{helpers::centered_text, Element};
use iced::widget::{checkbox, column, horizontal_rule, row, text_input};
use iced::{Length, Task};

use crate::utils::folder_dialog;

#[derive(Debug, Clone)]
pub enum Message {
    IndexOnly(bool),
    IndexRaw(bool),
    UpperCase(bool),
    LowerCase(bool),
    IndexPadding(u8),
    PreferFilename(bool),
    PrefixSamples(bool),

    ExportFormat(Format),
    SelfContained(bool),
    StrictLoad(bool),
    WorkerThreads(Workers),
    FolderDepth(u8),
    Destination(Option<PathBuf>),
    DestinationDialog,
}

/// Configure the behaviour of cfg samples
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ExtractionConfig {
    // Sample Naming
    pub index_raw: bool,
    pub index_only: bool,
    pub index_padding: u8,
    pub upper: bool,
    pub lower: bool,
    pub prefix: bool,
    pub prefer_filename: bool,
    // Custom filters
    // TODO

    // Sample cfg
    pub destination: PathBuf,
    pub self_contained: bool,
    pub folder_max_depth: u8,
    pub strict: bool,
    pub worker_threads: u8,
    pub exported_format: Format,
}

pub fn update(cfg: &mut ExtractionConfig, message: Message) -> Task<Message> {
    match message {
        Message::IndexOnly(index_only) => {
            if index_only {
                cfg.lower = false;
                cfg.upper = false;
                cfg.prefer_filename = false;
            }
            cfg.index_only = index_only;
        }
        Message::IndexRaw(b) => cfg.index_raw = b,
        Message::UpperCase(upper) => {
            if cfg.lower && upper {
                cfg.lower = false;
            }
            if upper {
                cfg.index_only = false;
            }
            cfg.upper = upper;
        }
        Message::LowerCase(lower) => {
            if cfg.upper && lower {
                cfg.upper = false;
            }
            if lower {
                cfg.index_only = false;
            }
            cfg.lower = lower;
        }
        Message::PreferFilename(use_filename) => {
            if use_filename {
                cfg.index_only = false;
            }
            cfg.prefer_filename = use_filename;
        }
        Message::IndexPadding(padding) => cfg.index_padding = padding,
        Message::PrefixSamples(prefix) => cfg.prefix = prefix,
        Message::ExportFormat(format) => cfg.exported_format = format,
        Message::SelfContained(toggle) => cfg.self_contained = toggle,
        Message::FolderDepth(depth) => cfg.folder_max_depth = depth,
        Message::StrictLoad(strict) => cfg.strict = strict,
        Message::WorkerThreads(Workers(threads)) => cfg.worker_threads = threads,
        Message::Destination(destination) => {
            if let Some(destination) = destination {
                cfg.destination = destination
            }
        }
        Message::DestinationDialog => {
            return Task::perform(folder_dialog(), Message::Destination);
        }
    }
    Task::none()
}

pub static DESTINATION_BAR_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub fn view_destination_bar(cfg: &ExtractionConfig) -> Element<Message> {
    let destination = cfg.destination.to_str().unwrap_or_default();

    let input = text_input("Output Directory", destination)
        .id(DESTINATION_BAR_ID.clone())
        .on_input(|f| {
            let destination = PathBuf::new().join(f);
            Message::Destination(Some(destination))
        });

    let button = centered_button("Open")
        .on_press(Message::DestinationDialog)
        .style(style::button::start);

    row![input, button]
        .spacing(5)
        .width(Length::FillPortion(1))
        .into()
}

pub fn view_naming(config: &ExtractionConfig, preview: impl ToString) -> Element<Message> {
    let col1 = column![
        checkbox("Index Only", config.index_only).on_toggle(Message::IndexOnly),
        checkbox("Preserve Index", config.index_raw).on_toggle(Message::IndexRaw),
        checkbox("Prefix Samples", config.prefix).on_toggle(Message::PrefixSamples),
    ]
    .spacing(8);

    let col2 = column![
        checkbox("Upper Case", config.upper).on_toggle(Message::UpperCase),
        checkbox("Lower Case", config.lower).on_toggle(Message::LowerCase),
        checkbox("Prefer Filename", config.prefer_filename,).on_toggle(Message::PreferFilename),
    ]
    .spacing(8);

    let checkboxes = row![col1, col2].spacing(8);
    let idx_padding = labelled_picklist(
        "Index Padding",
        [1, 2, 3, 4].as_slice(),
        Some(config.index_padding),
        Message::IndexPadding,
    );

    control(
        "Sample Naming",
        column![
            checkboxes,
            idx_padding,
            horizontal_rule(1),
            centered_column_x(column![centered_text(preview.to_string())])
        ]
        .spacing(8),
    )
    .into()
}

pub fn view_ripping(cfg: &ExtractionConfig) -> Element<Message> {
    let col1 = column![
        checkbox("Self Contained", cfg.self_contained).on_toggle(Message::SelfContained),
        checkbox("Strict Loading", cfg.strict).on_toggle(Message::StrictLoad),
    ]
    .spacing(8);

    let export_format = labelled_picklist(
        "Export Format",
        data::SUPPORTED_FORMATS,
        Some(cfg.exported_format),
        Message::ExportFormat,
    );

    let folder_scan_depth = labelled_picklist(
        "Folder Scan Depth",
        [1, 2, 3, 4, 5, 6, 7].as_slice(),
        Some(cfg.folder_max_depth),
        Message::FolderDepth,
    );

    let worker_threads = labelled_picklist(
        "Worker Threads",
        [0u8, 1, 2, 4, 6, 8, 10, 12, 16].map(Workers),
        Some(Workers(cfg.worker_threads)),
        Message::WorkerThreads,
    );

    let settings = column![
        col1,
        export_format,
        horizontal_rule(1),
        folder_scan_depth,
        worker_threads,
    ]
    .spacing(8);

    control("Ripping Configuration", settings).into()
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
#[repr(transparent)]
pub struct Workers(pub u8);

impl std::fmt::Display for Workers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "Automatic"),
            n => write!(f, "{}", n),
        }
    }
}

pub fn destination_is_valid(cfg: &ExtractionConfig) -> bool {
    cfg.destination
        .parent()
        .is_some_and(|path| path.exists() && path != Path::new(""))
}
