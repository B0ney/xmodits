use crate::gui::{style, JETBRAINS_MONO};
use crate::{core::cfg::SampleNameConfig, gui::style::Theme};
use iced::widget::{checkbox, column, container, pick_list, row, text};
use iced::{Alignment, Element, Length, Renderer};

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

impl SampleNameConfig {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::IndexOnly(index_only) => {
                if index_only {
                    self.lower = false;
                    self.upper = false;
                    self.prefer_filename = false;
                }
                self.index_only = index_only;
            }
            Message::IndexRaw(b) => self.index_raw = b,
            Message::UpperCase(upper) => {
                if self.lower && upper {
                    self.lower = false;
                }
                if upper {
                    self.index_only = false;
                }
                self.upper = upper;
            }
            Message::LowerCase(lower) => {
                if self.upper && lower {
                    self.upper = false;
                }
                if lower {
                    self.index_only = false;
                }
                self.lower = lower;
            }
            Message::PreferFilename(use_filename) => {
                if use_filename {
                    self.index_only = false;
                }
                self.prefer_filename = use_filename;
            },
            Message::IndexPadding(padding) => self.index_padding = padding,
            Message::PrefixSamples(prefix) => self.prefix = prefix,
        }
    }
    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let settings: _ = container(
            column![
                row![
                    column![
                        checkbox("Index Only", self.index_only, Message::IndexOnly),
                        checkbox("Preserve Index", self.index_raw, Message::IndexRaw),
                        checkbox("Prefix Samples", self.prefix, Message::PrefixSamples),
                    ]
                    .spacing(8),
                    column![
                        checkbox("Upper Case", self.upper, Message::UpperCase),
                        checkbox("Lower Case", self.lower, Message::LowerCase),
                        checkbox("Prefer Filename", self.prefer_filename, Message::PreferFilename),
                    ]
                    .spacing(8)
                ]
                .spacing(8),
                row![
                    pick_list(
                        (1..4).collect::<Vec<u8>>(),
                        Some(self.index_padding),
                        Message::IndexPadding
                    ),
                    "Index Padding"
                ]
                .align_items(Alignment::Center)
                .spacing(5),
            ]
            .spacing(8),
        )
        .style(style::Container::Frame)
        .padding(8)
        .width(Length::Fill);

        container(column![text("Sample Naming").font(JETBRAINS_MONO), settings].spacing(10))
            .width(Length::Fill)
            .into()
    }
}
