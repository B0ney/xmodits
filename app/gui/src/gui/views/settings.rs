use crate::gui::JETBRAINS_MONO;
use crate::{
    core::cfg::Config,
    gui::style::{self, Theme},
};
use iced::widget::button;
use iced::widget::{checkbox, column, pick_list, row, text};
use iced::{widget::container, Element, Length, Renderer};

#[derive(Debug, Clone)]
pub enum Message {
    ToggleSFX,
    // None,
    SFX(String),
}

#[derive(Default, Debug, Clone)]
pub struct SettingsView {
    pub sfx: bool,
    pub auto_update: bool,
    pub t1: bool,
    pub t2: bool,
    pub t3: bool,
    pub t4: bool,
    pub t5: bool,
}

impl SettingsView {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::ToggleSFX => self.sfx = !self.sfx,
            Message::SFX(_) => (),
        }
    }
    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let settings: _ = container(
            column![
                checkbox("SFX", self.sfx, |_| Message::ToggleSFX),
                // checkbox("Test1", self.t1, |_| Message::ToggleSFX),
                // checkbox("Test2", self.t2, |_| Message::ToggleSFX),
                // checkbox("Test3", self.t3, |_| Message::ToggleSFX),
                // checkbox("Test4", self.t4, |_| Message::ToggleSFX),
                // checkbox("Test5", self.t5, |_| Message::ToggleSFX),
                button("Test")
                    .on_press(Message::SFX("sfx_3".into()))
                    .padding(10),
                button("Test")
                    .on_press(Message::SFX("sfx_4".into()))
                    .padding(10)
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
