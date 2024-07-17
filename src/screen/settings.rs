use data::config::{self};
use iced::widget::{checkbox, column, pick_list, row};
use iced::Task;

use crate::widget::helpers::control;
use crate::widget::Element;

#[derive(Debug, Clone)]
pub enum Message {
    NonGuiQuietOutput(bool),
    NonGuiUseCwd(bool),
    ShowAnimatedGIF(bool),
    SuppressWarnings(bool),
    ShowErrorsInTextEditor(bool),
    SetTheme(data::theme::Themes),
}

/*
TODO:
    * Sample name preview parameters
    * Load custom animation for idle, ripping, and done states
    * Load custom themes and pick a preset
*/
pub fn view(general: &config::GeneralConfig) -> Element<Message> {
    #[cfg(feature = "iced_gif")]
    let hide_gif =
        Some(checkbox("Hide Animated GIFs", general.hide_gif).on_toggle(Message::ShowAnimatedGIF));

    #[cfg(not(feature = "iced_gif"))]
    let hide_gif = None::<Element<Message>>;

    let settings = column![]
        .push(
            checkbox("Hide Warnings", general.suppress_warnings)
                .on_toggle(Message::SuppressWarnings),
        )
        .push_maybe(hide_gif)
        .push(
            checkbox(
                "Open Saved Errors in Text Editor",
                general.show_errors_in_text_editor,
            )
            .on_toggle(Message::ShowErrorsInTextEditor),
        )
        .spacing(8);

    column![control("Application Settings", settings)]
        .push(themes(general))
        .push_maybe(non_gui(general))
        .spacing(8)
        .into()
}

pub fn themes(general: &config::GeneralConfig) -> Element<Message> {
    let settings = row![
        pick_list(
            data::theme::Themes::ALL.as_slice(),
            Some(general.theme),
            Message::SetTheme
        ),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);
    column![control("Themes", settings)].spacing(8).into()
}

#[cfg(target_env = "msvc")]
pub fn non_gui(general: &config::GeneralConfig) -> Option<Element<Message>> {
    let settings = column![
        checkbox("Quiet Output", general.non_gui_quiet_output)
            .on_toggle(Message::NonGuiQuietOutput),
        checkbox("Use Current Working Directory", general.non_gui_use_cwd,)
            .on_toggle(Message::NonGuiUseCwd),
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

pub fn update(cfg: &mut config::GeneralConfig, message: Message) -> Task<Message> {
    tracing::info!("{:?}", &message);

    match message {
        Message::NonGuiQuietOutput(quiet_output) => cfg.non_gui_quiet_output = quiet_output,
        Message::NonGuiUseCwd(use_cwd) => cfg.non_gui_use_cwd = use_cwd,
        Message::ShowAnimatedGIF(toggle) => cfg.hide_gif = toggle,
        Message::SuppressWarnings(toggle) => cfg.suppress_warnings = toggle,
        Message::SetTheme(theme) => cfg.theme = theme,
        Message::ShowErrorsInTextEditor(show) => cfg.show_errors_in_text_editor = show,
    }

    Task::none()
}
