use std::path::PathBuf;

use data::config::GeneralConfig;

use iced::widget::{checkbox, column, container, text};
use iced::Element;

#[derive(Debug, Clone)]
pub enum Message {
    SetLogDirectory(PathBuf),
    NonGuiQuietOutput(bool),
    NonGuiUseCwd(bool),
}

pub fn view<'a>(general: &'a GeneralConfig) -> Element<'a, Message> {
    let settings = column![
        checkbox(
            "(non-gui) Quiet output",
            general.non_gui_quiet_output,
            Message::NonGuiQuietOutput
        ),
        checkbox(
            "(non-gui) Use current working directory",
            general.non_gui_use_cwd,
            Message::NonGuiUseCwd
        ),
    ];

    let settings = column![text("Settings"), settings];

    container(settings).into()
}

pub fn update(cfg: &mut GeneralConfig, message: Message) {
    match message {
        Message::SetLogDirectory(log_path) => cfg.logging_path = Some(log_path),
        Message::NonGuiQuietOutput(quiet_output) => cfg.non_gui_quiet_output = quiet_output,
        Message::NonGuiUseCwd(use_cwd) => cfg.non_gui_use_cwd = use_cwd,
    }
}
