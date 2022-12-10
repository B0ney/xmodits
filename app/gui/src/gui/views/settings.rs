use std::path::PathBuf;

use crate::core::cfg::GeneralConfig;
use crate::gui::style::{self, Theme};
use crate::gui::JETBRAINS_MONO;
use iced::widget::button;
use iced::widget::{checkbox, column, pick_list, row, text};
use iced::{widget::container, Element, Length, Renderer};

#[derive(Debug, Clone)]
pub enum Message {
    ToggleSFX(bool),
    SetRecursionDepth(u8),
    SetLogPath(Option<PathBuf>),
    ToggleQuietOutput(bool),
}

impl GeneralConfig {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::ToggleSFX(b) => self.sfx = b,
            Message::SetRecursionDepth(depth) => self.folder_recursion_depth = depth,
            Message::SetLogPath(path) => self.logging_path = path,
            Message::ToggleQuietOutput(b) => self.quiet_output = b,
        }
    }

    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let settings: _ = container(
            column![
                checkbox("SFX", self.sfx, Message::ToggleSFX),
                checkbox(
                    "Quiet Output",
                    self.quiet_output,
                    Message::ToggleQuietOutput
                ),
            ]
            .spacing(5),
        )
        .style(style::Container::Frame)
        .padding(8)
        .width(Length::Fill);

        container(column![text("Settings").font(JETBRAINS_MONO), settings].spacing(10))
            .width(Length::Fill)
            .into()
    }
}
