use std::path::PathBuf;

use crate::gui::{style, JETBRAINS_MONO};
use crate::{core::cfg::SampleNameConfig, gui::style::Theme};
use iced::widget::Space;
use iced::widget::{checkbox, column, pick_list, row, text};
use iced::{widget::container, Alignment, Element, Length, Renderer};

#[derive(Debug, Clone)]
pub enum Message {
    IndexOnly(bool),
    IndexRaw(bool),
    UpperCase(bool),
    LowerCase(bool),
    IndexPadding(u8),
}

impl SampleNameConfig {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::IndexOnly(index_only) => {
                if index_only {
                    self.lower = false;
                    self.upper = false;
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
            Message::IndexPadding(padding) => self.index_padding = padding,
        }
    }
    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let settings: _ = container(
            row![
                column![
                    checkbox("Index Only", self.index_only, Message::IndexOnly),
                    checkbox("Preserve Index", self.index_raw, Message::IndexRaw)
                ]
                .spacing(8),
                column![
                    checkbox("Upper Case", self.upper, Message::UpperCase),
                    checkbox("Lower Case", self.lower, Message::LowerCase)
                ]
                .spacing(8)
            ]
            .spacing(8),
        )
        .style(style::Container::Frame)
        .padding(8)
        .width(Length::Fill);

        container(column![text("Ripping Configuration").font(JETBRAINS_MONO), settings].spacing(10))
            .width(Length::Fill)
            .into()
    }
}
