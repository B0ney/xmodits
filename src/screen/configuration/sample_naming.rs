//! Configure how samples should be named

use data::{config::SampleNameConfig, xmodits_lib::exporter::AudioFormat};

use crate::widget::Element;
use iced::widget::{checkbox, column, container, horizontal_rule, pick_list, row, text};

#[derive(Debug, Clone)]
pub enum Message {
    IndexOnly(bool),
    IndexRaw(bool),
    UpperCase(bool),
    LowerCase(bool),
    IndexPadding(u8),
    PreferFilename(bool),
    PrefixSamples(bool),
}

pub fn update(cfg: &mut SampleNameConfig, message: Message) {
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
    }
}

pub fn view<'a>(
    config: &'a SampleNameConfig,
    export_format: &'a AudioFormat,
    preview_name: &dyn Fn(&SampleNameConfig, &AudioFormat) -> String, // TODO
) -> Element<'a, Message> {
    let col1 = column![
        checkbox("Index Only", config.index_only, Message::IndexOnly),
        checkbox("Preserve Index", config.index_raw, Message::IndexRaw),
        checkbox("Prefix Samples", config.prefix, Message::PrefixSamples),
    ];

    let col2 = column![
        checkbox("Upper Case", config.upper, Message::UpperCase),
        checkbox("Lower Case", config.lower, Message::LowerCase),
        checkbox(
            "Prefer Filename",
            config.prefer_filename,
            Message::PreferFilename
        ),
    ];

    let checkboxes = row![col1, col2];

    let options: &[u8] = &[1, 2, 3, 4];
    let idx_padding = row![
        pick_list(options, Some(config.index_padding), Message::IndexPadding),
        "Index Padding"
    ];

    let settings = column![
        checkboxes,
        idx_padding,
        horizontal_rule(1),
        text(preview_name(config, export_format))
    ];

    let settings = column![text("Sample Naming"), settings];

    container(settings).into()
}
