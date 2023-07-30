use crate::gui::icons::folder_icon;
use crate::gui::style::{self, Theme};
use crate::gui::JETBRAINS_MONO;
use iced::widget::{column, container, text};
use iced::{Element, Length, Renderer};

use crate::gui::Message;

pub fn view() -> Element<'static, Message, Renderer<Theme>> {
    let about: _ = container(column![text("Xmodits - by B0ney"), folder_icon()])
        .style(style::Container::Frame)
        .padding(8)
        .height(Length::Fill)
        .width(Length::Fill);

    container(column![text("Help").font(JETBRAINS_MONO), about].spacing(15))
        .width(Length::Fill)
        .into()
}
