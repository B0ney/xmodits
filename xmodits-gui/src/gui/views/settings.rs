// use std::path::PathBuf;
use crate::core::cfg::GeneralConfig;
use crate::gui::style::{self, Theme};
use crate::gui::{JETBRAINS_MONO, Message};
use iced::widget::{checkbox, column, pick_list, row, text};
use iced::Alignment;
use iced::{widget::container, Element, Length, Renderer};

// #[derive(Debug, Clone)]
// pub enum Message {
//     // ToggleSFX(bool),
//     // SetRecursionDepth(u8),
//     // SetLogPath(Option<PathBuf>),
//     // ToggleQuietOutput(bool),
// }

// impl GeneralConfig {
    // pub fn update(&mut self, _: Message) {
    //     // match msg {
    //     //     // Message::ToggleSFX(b) => self.sfx = b,
    //     //     // Message::SetLogPath(path) => self.logging_path = path,
    //     //     // Message::ToggleQuietOutput(b) => self.quiet_output = b,
    //     // }
    // }

    // pub fn view(&self) -> Element<Message, Renderer<Theme>> {
    //     let settings: _ = container(
    //             row![
    //                 pick_list(
    //                     (1..=4).collect::<Vec<u8>>(),
    //                     Some(self.folder_recursion_depth),
    //                     Message::SetRecursionDepth
    //                 ),
    //                 text("Folder Scan Depth "),
    //                 // .width(Length::Units(50))
    //             ]
    //             .align_items(Alignment::Center)
    //             .spacing(5),
    //     )
    //     .style(style::Container::Frame)
    //     .padding(8)
    //     .width(Length::Fill)
    //     .height(Length::Fill);

    //     container(settings)
    //         .width(Length::Fill)
    //         .into()
    // }
// }
pub fn view() -> Element<'static, Message, Renderer<Theme>> {
    let about: _ = container(column![text("Settings")])
        .style(style::Container::Frame)
        .padding(8)
        .height(Length::Fill)
        .width(Length::Fill);

    container(column![text("Help").font(JETBRAINS_MONO), about].spacing(15))
        .width(Length::Fill)
        .into()
}