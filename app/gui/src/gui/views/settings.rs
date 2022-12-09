use std::path::PathBuf;

use crate::core::cfg::GeneralConfig;
use crate::gui::JETBRAINS_MONO;
use crate::{
    gui::style::{self, Theme},
};
use iced::widget::button;
use iced::widget::{checkbox, column, pick_list, row, text};
use iced::{widget::container, Element, Length, Renderer};

#[derive(Debug, Clone)]
pub enum Message {
    ToggleSFX(bool),
    SetRecursionDepth(u8),
    SetLogPath(PathBuf),
    ToggleQuietOutput(bool),
}

pub fn view(general: &GeneralConfig) -> Element<Message, Renderer<Theme>> {
    let settings: _ = container(
        column![
            checkbox("SFX", general.sfx, Message::ToggleSFX),
            checkbox("Quiet Output", general.quiet_output, Message::ToggleQuietOutput),


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

pub fn update(general: &mut GeneralConfig, msg: Message) {
    match msg {
        Message::ToggleSFX(b) => general.sfx = b,
        Message::SetRecursionDepth(depth) => general.folder_recursion_depth = depth,
        Message::SetLogPath(path) => general.logging_path = path,
        Message::ToggleQuietOutput(b) => general.quiet_output = b,
    }
}       