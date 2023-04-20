// use std::path::PathBuf;
use crate::core::cfg::GeneralConfig;
use crate::gui::style::{self, Theme};
use crate::gui::{JETBRAINS_MONO, App, Message};
use iced::widget::{checkbox, column, pick_list, row, text};
use iced::Alignment;
use iced::{widget::container, Element, Length, Renderer};

impl App {
    pub fn view_settings(&self) -> Element< Message, Renderer<Theme>> {
        let about: _ = container(column![
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
    ])
    .style(style::Container::Frame)
    .padding(8)
    .height(Length::Fill)
    .width(Length::Fill);

    container(column![text("Settings").font(JETBRAINS_MONO), about].spacing(15))
        .width(Length::Fill)
        .into()
    }
}
