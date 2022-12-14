use crate::gui::{style, JETBRAINS_MONO};
use crate::{core::cfg::SampleRippingConfig, gui::style::Theme};
use crate::core::cfg::FormatHint;
use iced::Alignment;
use iced::widget::{checkbox, column, container, row, text, text_input, pick_list};
use iced::{Element, Length, Renderer};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    SetDestination(PathBuf),
    SetHint(FormatHint),
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
                row![
                    pick_list(
                        &FormatHint::ALL[..],
                        Some(self.hint),
                        Message::SetHint
                    )
                    // .width(Length::Units(50))
                    ,
                    text("Format Hinting"),
                ]
                .align_items(Alignment::Center)
                .spacing(5)
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
