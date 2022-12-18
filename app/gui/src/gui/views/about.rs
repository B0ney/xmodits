use crate::gui::icons::{github_icon, gpl3_icon};
use crate::gui::style::{self, Theme};
use crate::gui::JETBRAINS_MONO;
use iced::widget::{button, column, container, row, text};
use iced::{alignment::Horizontal, Alignment, Element, Length, Renderer};
// use tracing::warn;

#[derive(Debug, Clone)]
pub enum Message {
    GH,
}

pub fn update(msg: Message) {
    match msg {
        Message::GH => {
            if let Err(_) = open::that(env!("CARGO_PKG_REPOSITORY")) {
                // warn!("Could not open external link: {:?}", e)
            };
        }
    }
}

pub fn view() -> Element<'static, Message, Renderer<Theme>> {
    let title = text("XMODITS - by B0ney").font(JETBRAINS_MONO);
    let about = text("A tool to rip samples from various tracker modules.")
        .font(JETBRAINS_MONO)
        .horizontal_alignment(Horizontal::Center);
    let repo = button(text(format!("{}", env!("CARGO_PKG_REPOSITORY"))).font(JETBRAINS_MONO))
        .style(style::button::Button::Hyperlink)
        .on_press(Message::GH);
    let version: _ = text(format!("version: {}", env!("CARGO_PKG_VERSION"))).font(JETBRAINS_MONO);
    // let gh: _ = button(github_icon()).on_press(Message::GH);
    let about: _ = container(
        column![
            title,
            version,
            about,
            repo,
            gpl3_icon().width(Length::Units(150))
        ]
        .align_items(Alignment::Center)
        .spacing(5),
    )
    .style(style::Container::Frame)
    .padding(8)
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y();

    container(column![text("About").font(JETBRAINS_MONO), about].spacing(15))
        .width(Length::Fill)
        .into()
}
