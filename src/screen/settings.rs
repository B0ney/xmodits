use data::config;
use iced::widget::{checkbox, column, container, text, text_input};
use iced::Command;
use std::path::PathBuf;

use crate::utils::folder_dialog;
use crate::widget::helpers::{control, control_filled, labelled_picklist};
use crate::widget::{Collection, Element};

#[derive(Debug, Default)]
pub struct GeneralConfig(pub config::GeneralConfig);

impl GeneralConfig {
    pub fn view(&self) -> Element<Message> {
        view(&self.0)
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        update(&mut self.0, message)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NonGuiQuietOutput(bool),
    NonGuiUseCwd(bool),
    SetLogFolder(Option<PathBuf>),
    SetLogFolderDialog,
}

/* 
TODO: 
    * Sample name preview parameters 
    * Load custom animation for idle, ripping, and done states
    * Load custom themes and pick a preset
*/
pub fn view(general: &config::GeneralConfig) -> Element<Message> {
    let settings = column![
        // labelled_picklist("Themes", options, selected, on_selected)
    ];

    column![control_filled("Application Settings", settings)]
        .push_maybe(non_gui(general))
        .spacing(8)
        .into()
}

#[cfg(target_env = "msvc")]
pub fn non_gui(general: &config::GeneralConfig) -> Option<Element<Message>> {
    let settings = column![
        checkbox(
            "Quiet Output",
            general.non_gui_quiet_output,
            Message::NonGuiQuietOutput
        ),
        checkbox(
            "Use Current Working Directory",
            general.non_gui_use_cwd,
            Message::NonGuiUseCwd
        ),
    ]
    .spacing(8);

    Some(control("Drag and Drop Mode", settings).into())
}

#[cfg(not(target_env = "msvc"))]
pub fn non_gui(general: &config::GeneralConfig) -> Option<Element<Message>> {
    None
}

pub fn update(cfg: &mut config::GeneralConfig, message: Message) -> Command<Message> {
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
