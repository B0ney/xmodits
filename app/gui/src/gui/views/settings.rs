use iced::{Element, Renderer, widget::container, Length};
use iced::widget::{text, pick_list,checkbox,column, row};

use crate::{gui::style::{self, Theme}, core::cfg::Config};
use crate::gui::JETBRAINS_MONO;

#[derive(Debug, Clone)]
pub enum Message {
    ToggleSFX,
}

#[derive(Default, Debug, Clone)]
pub struct SettingsView {
    pub sfx: bool,
}

impl SettingsView {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::ToggleSFX => self.sfx = !self.sfx,
        }
    }
    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let settings: _ = container(column![
            checkbox("SFX", self.sfx, |_| Message::ToggleSFX),
        ])
        .style(style::Container::Frame)
        .padding(8)
        .width(Length::Fill);

        container(
            column![
                text("Settings").font(JETBRAINS_MONO),
                settings
            ]
            .spacing(5)
        )
        .width(Length::Fill)
        .into()
    }
}