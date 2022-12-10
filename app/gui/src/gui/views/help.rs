use crate::gui::icons::github_icon;
use crate::gui::style::{self, Theme};
use crate::gui::JETBRAINS_MONO;
use iced::widget::svg;
use iced::widget::{button, checkbox, column, pick_list, row, text};
use iced::{widget::container, Element, Length, Renderer};
use tracing::warn;

pub enum Message {}

pub fn view() -> Element<'static, Message, Renderer<Theme>> {
    let about: _ = container(column![text("Xmodits - by B0ney"),])
        .style(style::Container::Frame)
        .padding(8)
        .height(Length::Fill)
        .width(Length::Fill);

    container(column![text("Help").font(JETBRAINS_MONO), about].spacing(10))
        .width(Length::Fill)
        .into()
}
