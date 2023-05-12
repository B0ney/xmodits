// use std::path::PathBuf;
use crate::core::cfg::GeneralConfig;
use crate::gui::style::{self, Theme};
use crate::gui::{App, Message, JETBRAINS_MONO};
use iced::widget::{checkbox, column, pick_list, row, text};
use iced::widget::{container, slider, Rule};
use iced::Alignment;
use iced::{Element, Length, Renderer};

impl App {
    pub fn view_settings(&self) -> Element<Message, Renderer<Theme>> {
        let about: _ = container(
            column![
                // text("Settings"),
                row![
                    text("Theme"),
                    pick_list(
                        &Theme::ALL[..],
                        Some(self.general_config.theme),
                        Message::SetTheme
                    ),
                ]
                .align_items(Alignment::Center)
                .spacing(5),
                Rule::horizontal(1),
                row![
                    text("Theme"),
                    slider(0..=100, 0, |_| Message::Ignore),
                    // pick_list(
                    //     &Theme::ALL[..],
                    //     Some(self.general_config.theme),
                    //     Message::SetTheme
                    // ),
                ]
                .align_items(Alignment::Center)
                .spacing(5),
            ]
            .spacing(5),
        )
        .style(style::Container::Frame)
        .padding(8)
        .height(Length::Fill)
        .width(Length::Fill);

        container(column![text("Settings").font(JETBRAINS_MONO), about].spacing(15))
            .width(Length::Fill)
            .into()
    }
}
