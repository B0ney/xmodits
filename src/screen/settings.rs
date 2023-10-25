use data::config::{self, general};
use iced::widget::{button, checkbox, column, container, text, text_input, Space};
use iced::{Command, Length};
use std::path::PathBuf;

use crate::screen::config::name_preview;

use crate::utils::{filename, folder_dialog};
use crate::widget::helpers::{action, control, control_filled, labelled_picklist};
use crate::widget::{Collection, Element};

#[derive(Debug, Clone)]
pub enum Message {
    NonGuiQuietOutput(bool),
    NonGuiUseCwd(bool),
    SetLogFolder(Option<PathBuf>),
    SetLogFolderDialog,
    ShowAnimatedGIF(bool),
    SuppressWarnings(bool),
    SetGif {
        kind: GIFKind,
        path: Option<PathBuf>,
    },
    SetTheme(data::theme::Themes),
    NamePreview(name_preview::Message)
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
        labelled_picklist(
            "Theme",
            data::theme::Themes::ALL.as_slice(),
            Some(general.theme),
            Message::SetTheme
        ),
        checkbox(
            "Show Animated GIFs",
            general.show_gif,
            Message::ShowAnimatedGIF
        ),
        checkbox(
            "Suppress Warnings",
            general.suppress_warnings,
            Message::SuppressWarnings
        ),
    ]
    .spacing(8);

    column![control_filled("Application Settings", settings)]
        // .push(animated_gif(general))
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
pub fn non_gui(_general: &config::GeneralConfig) -> Option<Element<Message>> {
    None
}

#[derive(Debug, Clone, Copy)]
pub enum GIFKind {
    Idle,
    Ripping,
    Complete,
}

// pub fn animated_gif(general: &config::GeneralConfig) -> Element<Message> {
//     let settings = column![checkbox(
//         "Show Animated GIFs",
//         general.show_gif,
//         Message::ToggleAnimatedGIF
//     ),]
//     // .push(button(
//     //     general
//     //         .logging_path
//     //         .as_deref()
//     //         .map(filename)
//     //         .unwrap_or_default(),
//     // ))
//     .spacing(8);
//     control("Animated GIFs", settings).into()
// }

pub fn update(cfg: &mut config::GeneralConfig, message: Message) -> Command<Message> {
    tracing::info!("{:?}", &message);

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
        Message::ShowAnimatedGIF(toggle) => cfg.show_gif = toggle,
        Message::SuppressWarnings(toggle) => cfg.suppress_warnings = toggle,
        Message::SetGif { kind, path } => match kind {
            GIFKind::Idle => cfg.idle_gif = path,
            GIFKind::Ripping => cfg.ripping_gif = path,
            GIFKind::Complete => cfg.complete_gif = path,
        },
        Message::SetTheme(theme) => cfg.theme = theme,
        Message::NamePreview(msg) => name_preview::update(&mut cfg.sample_name_params, msg),
    }

    Command::none()
}
