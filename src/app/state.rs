use iced::Command;
use std::path::PathBuf;

use super::Message;
use crate::ripper::extraction::error_handler::{self, ErrorHandler};
use crate::ripper::subscription::CompleteState;
use crate::utils::create_file_dialog;

/// The current state of the application.
#[derive(Default, Debug, Clone)]
pub enum State {
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

impl State {
    pub fn update_progress(&mut self, new_progress: f32, new_errors: u64) {
        if let Self::Ripping { progress, errors, .. } = self {
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
        let State::Finished { state, .. } = &self else {
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
