use std::path::PathBuf;

use crate::gui::{JETBRAINS_MONO, style};
use crate::{
    core::cfg::SampleNameConfig,
    gui::style::Theme,
};
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

// impl CfgView {
//     pub fn update(&mut self, msg: Message) -> bool {
//         match msg {
//             Message::NoFolder(b) => self..cfg.no_folder = b,
//             Message::IndexOnly(b) => {
//                 if b {
//                     self.upper = false;
//                     self.lower = false;
//                 }
//                 self.index_only = b;
//             }
//             Message::IndexRaw(b) => self.index_raw = b,
//             Message::UpperCase(b) => {
//                 if self.lower && b {
//                     self.lower = false
//                 }
//                 if !self.index_only {
//                     self.upper = b;
//                 } else {
//                     return true;
//                 }
//             }
//             Message::LowerCase(b) => {
//                 if self.upper && b {
//                     self.upper = false
//                 }
//                 if !self.index_only {
//                     self.lower = b;
//                 } else {
//                     return true;
//                 }
//             }

//             Message::IndexPadding(padding) => self.cfg.index_padding = padding,
//             Message::DestinationFolder(destination) => self.cfg.destination = destination,
//         }
//         false
//     }

pub fn view(name_cfg: &SampleNameConfig) -> Element<Message, Renderer<Theme>> {
    let settings: _ = container(
        row![
            column![
                // checkbox("No Folder", self.cfg.no_folder, Message::NoFolder),
                checkbox("Index Only", name_cfg.index_only, Message::IndexOnly),
                checkbox("Preserve Index", name_cfg.index_raw, Message::IndexRaw),
            ]
            .spacing(8),
            column![
                checkbox("Upper Case", name_cfg.upper, Message::UpperCase),
                checkbox("Lower Case", name_cfg.lower, Message::LowerCase),
                // row![
                //     pick_list(
                //         vec![1, 2, 3],
                //         Some(name_cfg.index_padding),
                //         Message::IndexPadding
                //     )
                //     .width(Length::Units(50)),
                //     // Space::with_width(Length::FillPortion(4)),
                //     text("Padding"),
                // ]
                // .spacing(5)
                // .align_items(Alignment::Center),
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

pub fn update(config: &mut SampleNameConfig, msg: Message) {
    match msg {
        Message::IndexOnly(b) => config.index_only = b,
        Message::IndexRaw(b) => config.index_raw = b,
        Message::UpperCase(upper) => config.upper = upper,
        Message::LowerCase(lower) => config.lower = lower,
        Message::IndexPadding(padding) => config.index_padding = padding,
    }
} 
