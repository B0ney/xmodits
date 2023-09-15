use data::config::GeneralConfig;
use iced::widget::{checkbox, column, container, text};
use iced::{Command, Element};
use std::path::PathBuf;

use crate::utils::folder_dialog;

#[derive(Debug, Clone)]
pub enum Message {
    NonGuiQuietOutput(bool),
    NonGuiUseCwd(bool),
    SetLogFolder(Option<PathBuf>),
    SetLogFolderDialog,
}

pub fn view(general: &GeneralConfig) -> Element<Message> {
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

pub fn update(cfg: &mut GeneralConfig, message: Message) -> Command<Message> {
    match message {
        Message::NonGuiQuietOutput(quiet_output) => cfg.non_gui_quiet_output = quiet_output,
        Message::NonGuiUseCwd(use_cwd) => cfg.non_gui_use_cwd = use_cwd,
        Message::SetLogFolder(log_path) => {
            if let Some(log_path) = log_path {
                cfg.logging_path = Some(log_path)
            }
        }
        Message::SetLogFolderDialog => {
            return Command::perform(folder_dialog(), Message::SetLogFolder)
        }
    }

    Command::none()
}
