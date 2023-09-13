//! The main display panel

pub mod entry;

use data::time::Time;
use entry::Entries;
use iced::{alignment::Horizontal, Alignment, Element, Length};

use super::tracker_info;
use crate::ripper::subscription::CompleteState;
use iced::widget::{
    button, checkbox, column, container, lazy, progress_bar, row, scrollable, text, text_input,
    Space,
};

#[derive(Debug, Clone)]
pub enum Message {
    SetState(),
    Ignore,
}

#[derive(Default, Debug, Clone)]
pub enum State {
    #[default]
    Idle,
    Ripping {
        message: Option<String>,
        progress: f32,
        total_errors: u64,
    },
    Finished {
        state: CompleteState,
        time: Time,
    },
    // SamplePreview,
}

// #[derive(Default, Debug, Clone, Copy)]
// pub enum CompleteState {
//     #[default]
//     NoError,
// }

pub fn view() {}

pub struct TrackerView {
    pub state: State,
    pub entries: Entries,
}

impl TrackerView {
    pub fn view(&self) -> Element<Message> {
        match &self.state {
            State::Idle => self.view_entries(),
            State::Ripping {
                message,
                progress,
                total_errors,
            } => view_ripping(message, *progress, *total_errors),
            State::Finished { state, time } => view_finished(state, time),
        }
    }

    pub fn update(&mut self) {}

    /// View the entries added by the user
    fn view_entries(&self) -> Element<Message> {
        if self.entries.is_empty() {
            return container(
                column![text("Drag and Drop")], // , gif(&GIF.idle)]
                                                //     .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into();
        }

        // self.entries

        todo!()
    }
}

fn view_ripping(message: &Option<String>, progress: f32, total_errors: u64) -> Element<Message> {
    container(
        column![
            text(match message.as_ref() {
                Some(info) => info,
                None => "Ripping...",
            })
            //
            .horizontal_alignment(Horizontal::Center),
            progress_bar(0.0..=100.0, progress).height(5).width(200),
            // cancel_ripping_button,
            // gif(&GIF.ripping)
        ]
        .spacing(8)
        .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

/// XMODITS has finished extracting the samples
fn view_finished<'a>(complete_state: &'a CompleteState, time: &'a Time) -> Element<'a, Message> {
    let continue_button = button("Continue")
        // .on_press(Message::SetState(State::Idle))
        // .on_press(Message::Ignore)
        .padding(5);

    let save_errors_button = button("Save Errors")
        // .on_press(Message::SaveErrors)
        .padding(5);

    match complete_state {
        CompleteState::NoErrors => container(
            column![
                text("Cancelled"),
                text("Drag and Drop"),
                text(time),
                Space::with_height(15),
                continue_button
            ]
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into(),
        CompleteState::Cancelled => container(
            column![
                text("Cancelled"),
                text("Drag and Drop"),
                text(time),
                Space::with_height(15),
                continue_button
            ]
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into(),
        CompleteState::SomeErrors(errors) => {
            let message = column![
                text("Done... But xmodits could not rip everything... (._.)")
                    .horizontal_alignment(Horizontal::Center),
                text("took...")
            ];

            let buttons = row![continue_button, save_errors_button]
                .padding(4)
                .spacing(6)
                .align_items(Alignment::Center);

            let errors = scrollable(column(
                errors
                    .iter()
                    .map(|error| {
                        let err = column![text(error.filename()), text(&error.reason)];
                        container(err).into()
                    })
                    .collect(),
            ));

            let view = column![message, buttons, errors,];

            container(view)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        }
        CompleteState::TooMuchErrors { log, total } => {
            container(
                column![
                    text("Done..."),
                    text("But there's too many errors to display! (-_-')"),
                    text("Check the logs at:"),
                    button(text(log.display()).horizontal_alignment(Horizontal::Center)).padding(0), // .on_press(Message::Open(log.to_owned()))
                    text(format!("{} errors written", total))
                        .horizontal_alignment(Horizontal::Center),
                    text(time),
                    // .horizontal_alignment(Horizontal::Center),
                    // space,
                    row![continue_button]
                        .padding(4)
                        .align_items(Alignment::Center)
                ]
                .align_items(Alignment::Center)
                .padding(4)
                .spacing(6),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
        }
        CompleteState::TooMuchErrorsNoLog {
            reason,
            errors,
            discarded,
            manually_saved,
        } => {
            let error_state = match errors.len() {
                0 => match manually_saved {
                    false => text("Manually Saving errors..."),
                    true => text("Errors saved manually :D"),
                },
                n => text(format!("{} stored errors", n)),
            };

            let discarded_errors = match discarded {
                0 => text("No errors were discarded."),
                n => text(format!(
                    "I had to discard {} error(s) to save memory. >_<",
                    n
                )), // .style(style::text::Text::Error),
            };

            container(
                column![
                    text("Done..."),
                    text("But there's too many errors to display! (-_-')"),
                    text("...and I can't store them to a file either:"),
                    text(format!("\"{}\"", reason)).horizontal_alignment(Horizontal::Center),
                    // .style(style::text::Text::Error),
                    error_state,
                    discarded_errors,
                    match manually_saved {
                        true => row![continue_button],
                        false => row![continue_button, save_errors_button],
                    }
                    .padding(4)
                    .spacing(6)
                    .align_items(Alignment::Center),
                ]
                .align_items(Alignment::Center)
                .padding(4)
                .spacing(6),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
        }
    }
}
