//! Information about xmodits

use std::path::PathBuf;

use crate::icon::{self, vbee3, xmodits_logo};
use crate::widget::helpers::{
    centered_column_x, centered_container, centered_text, control, control_filled, text_icon,
};
use crate::widget::{Collection, Element};
use crate::{theme, utils};

use iced::widget::{button, column, row, text};
use iced::Command;

use super::build_info;

#[derive(Debug, Clone)]
pub enum Message {
    ExportBuildInfo(Option<PathBuf>),
    FileDialog,
    Open(String),
    Ignore,
}

pub fn view<'a>() -> Element<'a, Message> {
    let title = row![vbee3(), text("XMODITS - by B0ney"), vbee3()]
        .align_items(iced::Alignment::Center)
        .spacing(8);
    let about = centered_text("A tool to rip samples from various tracker modules.");
    let repo = button(text(env!("CARGO_PKG_REPOSITORY")))
        .on_press(Message::Open(String::from(env!("CARGO_PKG_REPOSITORY"))))
        .style(theme::Button::HyperlinkInverted);
    let version = centered_text(format!("version: {}", env!("CARGO_PKG_VERSION")));
    let image = xmodits_logo();
    let about =
        centered_container(centered_column_x(column![title, version, image, about, repo,])).padding(8);

    let about = control_filled("About", about);
    let build = build_info::view().map(|view| {
        control(
            "Build Information",
            column![
                view,
                button(text_icon("Export", icon::save()))
                    .style(theme::Button::Start)
                    .on_press(Message::FileDialog)
            ]
            .spacing(8),
        )
    });

    column![about].push_maybe(build).spacing(8).into()
}

pub fn update(msg: Message) -> Command<Message> {
    match msg {
        Message::ExportBuildInfo(Some(path)) => {
            return Command::perform(build_info::export_build(path.clone()), move |_| {
                Message::Open(path.display().to_string())
            })
        }
        Message::FileDialog => {
            let build_name = format!("xmodits-v{}-build-info", env!("CARGO_PKG_VERSION"));
            return Command::perform(utils::create_file_dialog(build_name), Message::ExportBuildInfo)
        }
        Message::Open(link) => {
            if let Err(e) = open::that_detached(link) {
                tracing::error!("{}", e.to_string());
            }
        }
        _ => (),
    }
    Command::none()
}
