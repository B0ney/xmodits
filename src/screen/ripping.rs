use data::time::Time;
use iced::Command;
use std::path::{Path, PathBuf};

use crate::app::Message;

use crate::ripper::extraction::error::Reason;
use crate::ripper::extraction::error_handler::{self, ErrorHandler};
use crate::ripper::subscription::CompleteState;
use crate::utils::create_file_dialog;
use crate::widget::helpers::{
    centered_column_x, centered_container, centered_text, fill_container, text_adv, text_icon,
};
use crate::widget::{self, Collection, Element};
use crate::{icon, theme};

use iced::widget::{
    button, column, container, progress_bar, row, scrollable, text, Space,
};
use iced::{Alignment, Length};

/// The current state of the application.
#[derive(Default, Debug, Clone)]
pub enum RippingState {
    #[default]
    Idle,
    /// The application is currently ripping samples
    Ripping {
        message: Option<String>,
        progress: f32,
        errors: u64,
    },
    /// The application has finished ripping samples
    Finished {
        state: CompleteState,
        time: data::Time,
        destination: PathBuf,
    },
}

impl RippingState {
    pub fn update_progress(&mut self, new_progress: f32, new_errors: u64) {
        if let Self::Ripping {
            progress, errors, ..
        } = self
        {
            *progress = new_progress;
            *errors = new_errors;
        }
    }

    pub fn update_message(&mut self, new_message: Option<String>) {
        if let Self::Ripping { message, .. } = self {
            *message = new_message
        }
    }

    pub fn set_message(&mut self, message: impl Into<String>) {
        self.update_message(Some(message.into()))
    }

    pub fn is_ripping(&self) -> bool {
        matches!(self, Self::Ripping { .. })
    }

    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Finished { .. })
    }

    pub fn export_errors(&mut self) -> Command<Message> {
        let RippingState::Finished { state, .. } = &self else {
            return Command::none();
        };

        let Some(errors) = state.errors_ref().cloned() else {
            return Command::none();
        };

        let task = async move {
            let Some(path) = create_file_dialog(error_handler::random_name()).await else {
                return Err(String::new()); // todo
            };

            ErrorHandler::dump(errors, path).await
        };

        Command::perform(task, Message::SaveErrorsResult)
    }
}

pub fn view_ripping(
    message: &Option<String>,
    progress: f32,
    total_errors: u64,
    show_gif: bool,
) -> Element<Message> {
    let cancel_ripping_button = button("CANCEL")
        .on_press(Message::Cancel)
        .style(theme::Button::Cancel)
        .padding(5);

    let view = column![
        text(message.as_deref().unwrap_or("Ripping...")),
        text(format!("{}% - Errors: {}", progress.floor(), total_errors)),
        progress_bar(0.0..=100.0, progress).height(5).width(200),
        cancel_ripping_button,
    ]
    .push_maybe(show_gif.then(|| widget::animation::GIF.ripping()).flatten())
    .spacing(8)
    .align_items(Alignment::Center);

    centered_container(view)
        .style(theme::Container::Black)
        .into()
}

/// XMODITS has finished extracting the samples
pub fn view_finished<'a>(
    complete_state: &'a CompleteState,
    time: &'a Time,
    hovered: bool,
    destination: &'a Path,
) -> Element<'a, Message> {
    let continue_button = button("Continue")
        .on_press(Message::SetState(RippingState::Idle))
        .style(theme::Button::Start)
        .padding(5);

    let save_errors_button = button(text_icon("Save Errors", icon::save()))
        .on_press(Message::SaveErrors)
        .padding(5);

    let open_destination_button = button(text_icon("Show Folder", icon::folder()))
        .on_press(Message::Open(destination.display().to_string()))
        .padding(5);

    match complete_state {
        CompleteState::NoErrors => centered_container(
            column![
                text("Done! \\(^_^)/"),
                text("Drag and Drop"),
                text(format!("{}", time)),
                Space::with_height(15),
                row![continue_button, open_destination_button].spacing(8)
            ]
            .align_items(Alignment::Center),
        )
        .style(theme::Container::BlackHovered(hovered))
        .into(),

        CompleteState::Cancelled => centered_container(
            column![
                text("Cancelled"),
                text("Drag and Drop"),
                text(format!("{}", time)),
                Space::with_height(15),
                continue_button
            ]
            .align_items(Alignment::Center),
        )
        .style(theme::Container::BlackHovered(hovered))
        .into(),

        CompleteState::Aborted => {
            let shutdown_button = button("Close Application")
                .on_press(Message::Shutdown)
                .style(theme::Button::Cancel)
                .padding(5);

            centered_container(
                column![
                    text("Ripping process was aborted because of an internal error."),
                    shutdown_button
                ]
                .spacing(4)
                .align_items(Alignment::Center)
            )
            .style(theme::Container::Black)
            .into()
        }

        CompleteState::SomeErrors(errors) => {
            let message = column![
                centered_text("Done... But xmodits could not rip everything..."),
                centered_text("(._.)"),
                centered_text(format!("{}", time)),
            ]
            .align_items(Alignment::Center);

            let buttons = row![continue_button, open_destination_button, save_errors_button]
                .padding(4)
                .spacing(6)
                .align_items(Alignment::Center);

            let errors = scrollable(
                column(errors.iter().map(|error| {
                    let reason = match &error.reason {
                        Reason::Single(single) => centered_text(single),
                        Reason::Multiple(_) => centered_text("multiple..."), //todo
                    };

                    let error = text_adv(error.filename());
                    let error = container(column![error, reason])
                        .padding(4)
                        .width(Length::Fill)
                        .style(theme::Container::Frame);
                    row![error, Space::with_width(15)].into()
                }))
                .spacing(8),
            );

            let view = centered_column_x(column![message, buttons, errors].padding(8));

            fill_container(view)
                .style(theme::Container::BlackHovered(hovered))
                .into()
        }

        CompleteState::TooMuchErrors { log, total } => {
            let view = column![
                text("Done..."),
                text("But there's too many errors to display! (-_-')"),
                text("Check the logs at:"),
                button(centered_text(log.display()))
                    .on_press(Message::Open(log.display().to_string()))
                    .style(theme::Button::HyperlinkInverted),
                centered_text(format!("{} errors written.", total)),
                centered_text(format!("{}.", time)),
                row![continue_button, open_destination_button]
                    .spacing(8)
                    .padding(4)
                    .align_items(Alignment::Center)
            ]
            .align_items(Alignment::Center)
            .padding(4)
            .spacing(6);

            centered_container(view)
                .style(theme::Container::BlackHovered(hovered))
                .into()
        }

        CompleteState::TooMuchErrorsNoLog {
            reason,
            errors,
            discarded,
        } => {
            let error_message = text(format!("{} stored errors", errors.len()));
            let discarded_errors = match discarded {
                0 => text("No errors were discarded."),
                n => text(format!(
                    "I had to discard {} error(s) to save memory. >_<",
                    n
                )),
            };

            let buttons = row![continue_button, open_destination_button, save_errors_button]
                .padding(4)
                .spacing(8)
                .align_items(Alignment::Center);

            let view = column![
                text("Done..."),
                text("But there's too many errors to display! (-_-')"),
                text("...and I can't store them to a file either:"),
                centered_text(format!("\"{}\"", reason)),
                error_message,
                discarded_errors,
                buttons,
            ]
            .align_items(Alignment::Center)
            .padding(4)
            .spacing(6);

            centered_container(view)
                .style(theme::Container::BlackHovered(hovered))
                .into()
        }
    }
}
