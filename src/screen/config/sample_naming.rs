//! Configure how samples should be named

use data::config::{self, SampleNameConfig};

use crate::theme;
use crate::widget::helpers::{control, labelled_picklist, centered_column_x};
use crate::widget::{helpers::centered_text, Element};

use iced::widget::{checkbox, column, container, horizontal_rule, pick_list, row, text};
use iced::{Alignment, Length};

#[derive(Debug, Default)]
pub struct NamingConfig(pub config::SampleNameConfig);

impl NamingConfig {
    pub fn update(&mut self, message: Message) {
        tracing::info!("{:?}", &message);
        update(&mut self.0, message)
    }

    pub fn view(&self, preview: impl ToString) -> Element<Message> {
        view(&self.0, preview)
    }
}

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

pub fn view<'a>(config: &'a SampleNameConfig, preview: impl ToString) -> Element<'a, Message> {
    let col1 = column![
        checkbox("Index Only", config.index_only, Message::IndexOnly),
        checkbox("Preserve Index", config.index_raw, Message::IndexRaw),
        checkbox("Prefix Samples", config.prefix, Message::PrefixSamples),
    ]
    .spacing(8);

    let col2 = column![
        checkbox("Upper Case", config.upper, Message::UpperCase),
        checkbox("Lower Case", config.lower, Message::LowerCase),
        checkbox(
            "Prefer Filename",
            config.prefer_filename,
            Message::PreferFilename
        ),
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
            centered_column_x(column![centered_text(preview)])
        ]
        .spacing(8),
    )
    .into()
}
