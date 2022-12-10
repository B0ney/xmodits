use crate::gui::{style, JETBRAINS_MONO};
use crate::{core::cfg::SampleRippingConfig, gui::style::Theme};
use iced::widget::{checkbox, column, pick_list, row, text};
use iced::widget::{text_input, Space};
use iced::{widget::container, Alignment, Element, Length, Renderer};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    SetDestination(PathBuf),
    SetHint(Option<String>),
    ToggleEmbedLoopPoint(bool),
    ToggleNoFolder(bool),
}

impl SampleRippingConfig {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::SetDestination(path) => self.destination = path,
            Message::SetHint(hint) => self.hint = hint,
            Message::ToggleEmbedLoopPoint(toggle) => self.embed_loop_points = toggle,
            Message::ToggleNoFolder(toggle) => self.no_folder = toggle,
        }
    }

    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let settings: _ = container(
            row![column![
                checkbox("No Folder", self.no_folder, Message::ToggleNoFolder),
                checkbox(
                    "Embed sample loops",
                    self.embed_loop_points,
                    Message::ToggleEmbedLoopPoint
                ),
                // row![
                //     pick_list(
                //         vec![Some("umx".to_string()), None],
                //         Some(config.hint),
                //         Message::IndexPadding
                //     )
                //     .width(Length::Units(50)),
                //     // Space::with_width(Length::FillPortion(4)),
                //     text("Padding"),
                // ]
                // .spacing(5)
            ]
            .spacing(8),]
            .spacing(8),
        )
        .style(style::Container::Frame)
        .padding(8)
        .width(Length::Fill);

        container(column![text("Ripping Configuration").font(JETBRAINS_MONO), settings].spacing(10))
            .width(Length::Fill)
            .into()
    }

    pub fn view_destination_bar(&self) -> Element<Message, Renderer<Theme>> {
        let input: _ = text_input(
            "Output Directory",
            &format!("{}", self.destination.display()),
            |s| Message::SetDestination(PathBuf::new().join(s)),
        )
        .padding(10);

        input.into()
    }
}
