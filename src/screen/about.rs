//! Information about xmodits

use crate::icon::{self, vbee3, xmodits_logo};
use crate::widget::helpers::{
    centered_column_x, centered_container, centered_text, control, control_filled, text_icon,
};
use crate::widget::Element;
use crate::{style, utils};

use iced::widget::{button, column, row, text};
use iced::Task;

use super::build_info;

#[derive(Debug, Clone)]
pub enum Message {
    BuildInfo,
    Open(String),
    Manual,
    Ignore(()),
}

pub fn view() -> Element<'static, Message> {
    let title = row![vbee3(), text("XMODITS - by B0ney"), vbee3()]
        .align_y(iced::Alignment::Center)
        .spacing(8);

    let repo = button(text(env!("CARGO_PKG_REPOSITORY")))
        .on_press(Message::Open(String::from(env!("CARGO_PKG_REPOSITORY"))))
        .style(style::button::hyperlink_inverted);

    #[cfg(feature = "manual")]
    let manual_button = Some(
        button(text("Manual"))
            .style(style::button::start)
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
                    .style(style::button::start)
                    .on_press(Message::BuildInfo)
            ]
            .spacing(8),
        )
    });

    column![about].push_maybe(build).spacing(8).into()
}

pub fn update(msg: Message) -> Task<Message> {
    match msg {
        Message::BuildInfo => Task::perform(export_build_info(), Message::Ignore),
        Message::Manual => Task::perform(export_manual(), Message::Ignore),
        Message::Open(link) => {
            if let Err(e) = open::that_detached(link) {
                tracing::error!("{}", e.to_string());
            }
            Task::none()
        }
        _ => Task::none(),
    }
}

#[cfg(not(feature = "manual"))]
async fn export_manual() {}

#[cfg(feature = "manual")]
async fn export_manual() {
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    let manual_name = format!("xmodits-v{}-manual.txt", env!("CARGO_PKG_VERSION"));

    if let Some(path) = utils::create_file_dialog(manual_name).await {
        if let Ok(mut file) = File::create(&path).await {
            if file.write_all(data::MANUAL.as_bytes()).await.is_ok() {
                let _ = open::that_detached(path);
            };
        };
    }
}

async fn export_build_info() {
    let build_name = format!("xmodits-v{}-build-info.txt", env!("CARGO_PKG_VERSION"));

    if let Some(path) = utils::create_file_dialog(build_name).await {
        if build_info::export_build(&path).await.is_ok() {
            let _ = open::that_detached(path);
        }
    }
}
