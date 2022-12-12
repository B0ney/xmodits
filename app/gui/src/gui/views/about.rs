use crate::gui::icons::github_icon;
use crate::gui::style::{self, Theme};
use crate::gui::JETBRAINS_MONO;
use iced::widget::svg;
use iced::widget::{button, checkbox, column, pick_list, row, text};
use iced::{widget::container, Element, Length, Renderer};
use tracing::warn;

#[derive(Debug, Clone)]
pub enum Message {
    GH,
}

pub fn update(msg: Message) {
    match msg {
        Message::GH => {
            if let Err(e) = open::that("https://github.com/B0ney/xmodits") {
                warn!("Could not open external link: {:?}", e)
            };
        }
    }
}

pub fn view() -> Element<'static, Message, Renderer<Theme>> {
    let logo: _ = text("0.0.7-Alpha").font(JETBRAINS_MONO);
    let gh: _ = button(github_icon().size(20)).on_press(Message::GH);
    let about: _ = container(column![text("Xmodits - by B0ney"), logo, gh,])
        .style(style::Container::Frame)
        .padding(8)
        .width(Length::Fill)
        .height(Length::Fill);

    container(column![text("About").font(JETBRAINS_MONO), about].spacing(15))
        .width(Length::Fill)
        .into()
}
