//! Information about xmodits

use crate::app::Message;
use crate::icon::xmodits_logo;
use crate::theme;
use crate::widget::helpers::{
    centered_column_x, centered_container, centered_text, control, control_filled,
};
use crate::widget::{Collection, Element};
use iced::widget::{button, column, image, text};

use super::build_info;

pub fn view<'a>() -> Element<'a, Message> {
    let title = centered_text("XMODITS - by B0ney");
    let about = centered_text("A tool to rip samples from various tracker modules.");
    let repo = button(text(env!("CARGO_PKG_REPOSITORY")))
        .on_press(Message::Open(String::from(env!("CARGO_PKG_REPOSITORY"))))
        .style(theme::Button::Hyperlink);
    let version = centered_text(format!("version: {}", env!("CARGO_PKG_VERSION")));
    let image = image(xmodits_logo());
    let about = centered_container(centered_column_x(column![
        title, version, image, about, repo,
    ]))
    .padding(8);

    let about = control_filled("About", about);
    let build = build_info::view().map(|view| control("Build Information", view));

    column![about].push_maybe(build).spacing(8).into()
}
