//! Information about xmodits

use crate::widget::Element;
use iced::widget::{button, column, container, text};

use super::build_info;

use tracing::warn;

#[derive(Debug, Clone)]
pub enum Message {
    GitHub,
}

pub fn update(msg: Message) {
    match msg {
        Message::GitHub => {
            if let Err(err) = open::that_detached(env!("CARGO_PKG_REPOSITORY")) {
                warn!("Could not open external link: {:?}", err)
            };
        }
    }
}

pub fn view<'a>() -> Element<'a, Message> {
    let title = text("XMODITS - by B0ney");
    let about = text("A tool to rip samples from various tracker modules.");
    let repo = button(text(env!("CARGO_PKG_REPOSITORY"))).on_press(Message::GitHub);
    let version = text(format!("version: {}", env!("CARGO_PKG_VERSION")));

    let about = container(column![title, version, about, repo])
        .padding(8)
        .center_x()
        .center_y();

    container(column![text("About"), about]).into()
}
