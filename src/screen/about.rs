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
    BuildInfo,
    Open(String),
    Manual,
    Ignore(()),
}

pub fn view<'a>() -> Element<'a, Message> {
    let title = row![vbee3(), text("XMODITS - by B0ney"), vbee3()]
        .align_items(iced::Alignment::Center)
        .spacing(8);

    let repo = button(text(env!("CARGO_PKG_REPOSITORY")))
        .on_press(Message::Open(String::from(env!("CARGO_PKG_REPOSITORY"))))
        .style(theme::Button::HyperlinkInverted);

    #[cfg(feature = "manual")]
    let manual_button = Some(
        button(text("Manual"))
            .style(theme::Button::Start)
            .on_press(Message::Manual),
    );

    #[cfg(not(feature = "manual"))]
    let manual_button = None::<Element<Message>>;

    let about = centered_container(centered_column_x(column![
        title,
        centered_text(format!("version: {}", env!("CARGO_PKG_VERSION"))),
        xmodits_logo(),
        centered_text("A tool to rip samples from various tracker modules."),
        repo,
    ]))
    .padding(8);

    let about = control_filled("About", column![about].push_maybe(manual_button));

    let build = build_info::view().map(|view| {
        control(
            "Build Information",
            column![
                view,
                button(text_icon("Export", icon::save()))
                    .style(theme::Button::Start)
                    .on_press(Message::BuildInfo)
            ]
            .spacing(8),
        )
    });

    column![about].push_maybe(build).spacing(8).into()
}

pub fn update(msg: Message) -> Command<Message> {
    match msg {
        Message::BuildInfo => return Command::perform(export_build_info(), Message::Ignore),
        Message::Manual => return Command::perform(export_manual(), Message::Ignore),
        Message::Open(link) => {
            if let Err(e) = open::that_detached(link) {
                tracing::error!("{}", e.to_string());
            }
        }
        _ => (),
    }
    Command::none()
}

#[cfg(not(feature = "manual"))]
async fn export_manual() {}

#[cfg(feature = "manual")]
async fn export_manual() {
    use tokio::io::AsyncWriteExt;
    use tokio::fs::File;

    let build_name = format!("xmodits-v{}-manual", env!("CARGO_PKG_VERSION"));

    if let Some(path) = utils::create_file_dialog(build_name).await {
        if let Ok(mut file) = File::create(&path).await {
            if file.write_all(data::MANUAL.as_bytes()).await.is_ok() {
                let _ = open::that_detached(path);
            };
        };
    }
}

async fn export_build_info() {
    let build_name = format!("xmodits-v{}-build-info", env!("CARGO_PKG_VERSION"));

    if let Some(path) = utils::create_file_dialog(build_name).await {
        if build_info::export_build(&path).await.is_ok() {
            let _ = open::that_detached(path);
        }
    }
}
