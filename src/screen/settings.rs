use data::config::{self};
use iced::widget::{button, checkbox, column, container, horizontal_rule, pick_list, row, Space};
use iced::{Alignment, Command, Length};
use std::path::{Path, PathBuf};

use crate::screen::config::name_preview;

use crate::theme;
use crate::utils::{file_dialog, filename, files_dialog, folder_dialog};
use crate::widget::helpers::{control, control_filled, labelled_picklist};
use crate::widget::{Collection, Element};

#[derive(Debug, Clone)]
pub enum Message {
    NonGuiQuietOutput(bool),
    NonGuiUseCwd(bool),
    SetLogFolder(Option<PathBuf>),
    SetLogFolderDialog,
    ShowAnimatedGIF(bool),
    SuppressWarnings(bool),
    SetGif { kind: GIFKind, action: SetGifAction },
    SetGifDialog { kind: GIFKind },
    SetTheme(data::theme::Themes),
    LoadThemeDialog,
    NamePreview(name_preview::Message),
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
        checkbox(
            "Suppress Warnings",
            general.suppress_warnings,
            Message::SuppressWarnings
        ),
    ]
    .spacing(8);

    column![control("Application Settings", settings)]
        // .push(animated_gif(general))
        .push(view_customisation(general))
        .push_maybe(drag_and_drop_mode(general))
        .spacing(8)
        .into()
}

#[cfg(target_env = "msvc")]
pub fn drag_and_drop_mode(general: &config::GeneralConfig) -> Option<Element<Message>> {
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
pub fn drag_and_drop_mode(_general: &config::GeneralConfig) -> Option<Element<Message>> {
    None
}

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
        Message::SetLogFolderDialog => return Command::perform(folder_dialog(), Message::SetLogFolder),
        Message::ShowAnimatedGIF(toggle) => cfg.show_gif = toggle,
        Message::SuppressWarnings(toggle) => cfg.suppress_warnings = toggle,
        Message::SetGif { kind, action: path } => match path {
            SetGifAction::Delete => *get_gif_path(cfg, kind) = None,
            SetGifAction::SetPath(path) => {
                if let Some(path) = path {
                    *get_gif_path(cfg, kind) = Some(path)
                }
            }
        },
        Message::SetTheme(theme) => cfg.theme = theme,
        Message::LoadThemeDialog => (),
        Message::NamePreview(msg) => name_preview::update(&mut cfg.sample_name_params, msg),
        Message::SetGifDialog { kind } => {
            return Command::perform(file_dialog(), move |path| Message::SetGif {
                kind,
                action: SetGifAction::SetPath(path),
            })
        }
    }

    Command::none()
}

pub fn view_customisation(general: &config::GeneralConfig) -> Element<Message> {
    let settings = column![row![
        "Theme:",
        pick_list(
            data::theme::Themes::ALL.as_slice(),
            Some(general.theme),
            Message::SetTheme
        ),
        button("Import Theme").on_press(Message::LoadThemeDialog)
    ]
    .align_items(Alignment::Center)
    .spacing(8),]
    .push_maybe(view_gif_settings(&general))
    .spacing(8);
    control("Customisation", settings).into()
}

#[derive(Debug, Clone, Copy)]
pub enum GIFKind {
    Idle,
    Ripping,
    Complete,
}

#[derive(Debug, Clone)]
pub enum SetGifAction {
    Delete,
    SetPath(Option<PathBuf>),
}

fn get_gif_path(cfg: &mut config::GeneralConfig, kind: GIFKind) -> &mut Option<PathBuf> {
    match kind {
        GIFKind::Idle => &mut cfg.idle_gif,
        GIFKind::Ripping => &mut cfg.ripping_gif,
        GIFKind::Complete => &mut cfg.complete_gif,
    }
}

pub fn view_gif_settings<'a>(cfg: &'a config::GeneralConfig) -> Option<Element<Message>> {
    let selection = |label: &'a str, path: &'a Option<PathBuf>, kind: GIFKind| {
        let button_label = path.as_deref().map(filename).unwrap_or("[ BUILT-IN ]");
        row![
            label,
            button(button_label)
                .on_press(Message::SetGifDialog { kind })
                .style(theme::Button::HyperlinkInverted),
            Space::with_width(Length::Fill)
        ]
        .push_maybe(path.is_some().then(|| {
            button("X")
                .style(theme::Button::Delete)
                .on_press(Message::SetGif {
                    kind,
                    action: SetGifAction::Delete,
                })
        }))
        .align_items(iced::Alignment::Center)
    };

    // TODO: Have preview of GIFs, it could be through a tooltip
    let gif_selection = container(
        column![
            selection("Idle:", &cfg.idle_gif, GIFKind::Idle),
            selection("Ripping:", &cfg.ripping_gif, GIFKind::Ripping),
            selection("Complete:", &cfg.complete_gif, GIFKind::Complete),
        ]
        .spacing(8),
    )
    .padding(8)
    .style(theme::Container::Black)
    .width(Length::Fill);

    let settings = column![
        horizontal_rule(1),
        checkbox("Show Animated GIFs", cfg.show_gif, Message::ShowAnimatedGIF),
    ]
    .spacing(8)
    .push_maybe(cfg.show_gif.then(|| gif_selection))
    .into();
    Some(settings)
}

#[cfg(s)]
pub fn view_gif_settings(_cfg: &config::GeneralConfig) -> Option<Element<Message>> {
    None
}
