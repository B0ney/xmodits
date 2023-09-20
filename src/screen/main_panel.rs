//! The main display panel

pub mod entry;

use data::time::Time;
use entry::Entries;
use iced::{alignment::Horizontal, Alignment, Length};

use self::entry::Entry;

use super::tracker_info;
use crate::{icon, theme};
use crate::ripper::subscription::CompleteState;
use crate::theme::Button;
use crate::widget::helpers::text_centered;
use crate::widget::{Collection, Element};

use crate::app::Message;

use iced::widget::{
    button, checkbox, column, container, progress_bar, row, scrollable, text, Space,
};

pub fn view_samples<'a>() -> Element<'a, Message> {
    todo!()
}

pub fn view_entries(entries: &Entries) -> Element<Message> {
    let entries = &entries.entries;

    if entries.is_empty() {
        return container(text_centered("Drag and Drop"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into();
    }

    scrollable(column(
        entries
            .iter()
            .enumerate()
            .map(|(index, entry)| view_entry(index, entry))
            .collect(),
    ))
    .into()
}

fn view_entry(index: usize, entry: &Entry) -> Element<Message> {
    let check = checkbox("", entry.selected, move |selected| Message::Select {
        index,
        selected,
    });

    let filename = text(&entry.filename());

    let view = row![check, filename]
        .push_maybe(match entry.is_dir() {
            true => Some(row![Space::with_width(Length::Fill), icon::folder()]),
            false => None,
        })
        .spacing(1)
        .align_items(Alignment::Center);

    row![
        button(view)
            .width(Length::Fill)
            .on_press(Message::Probe(index))
            .padding(4)
            .style(theme::Button::Entry),
        Space::with_width(15)
    ]
    .into()
}

fn view_ripping<'a>(
    message: &Option<String>,
    progress: f32,
    total_errors: u64,
) -> Element<'a, Message> {
    let cancel_ripping_button = button("Continue").on_press(Message::Cancel).padding(5);

    let view = column![
        text_centered(message.as_deref().unwrap_or("Ripping...")),
        progress_bar(0.0..=100.0, progress).height(5).width(200),
        cancel_ripping_button,
        text_centered(format!("Errors: {}", total_errors)),
        // gif(&GIF.ripping)
    ]
    .spacing(8)
    .align_items(Alignment::Center);

    container(view)
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
        .padding(5);

    let save_errors_button = button("Save Errors")
        .on_press(Message::SaveErrors)
        .padding(5);

    match complete_state {
        CompleteState::NoErrors => container(
            column![
                text("Done! \\(^_^)/"),
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

        // TODO
        CompleteState::Aborted => container(
            column![
                text("An internal error occured."),
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
                text_centered("Done... But xmodits could not rip everything... (._.)"),
                text("took...")
            ];

            let buttons = row![continue_button, save_errors_button]
                .padding(4)
                .spacing(6)
                .align_items(Alignment::Center);

            // let errors = scrollable(column(
            //     errors
            //         .iter()
            //         .map(|error| {
            //             let err = column![text(error.filename()), text(&error.reason)];
            //             container(err).into()
            //         })
            //         .collect(),
            // ));

            let view = column![message, buttons,];

            container(view)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        }

        CompleteState::TooMuchErrors { log, total } => {
            let view = column![
                text("Done..."),
                text("But there's too many errors to display! (-_-')"),
                text("Check the logs at:"),
                button(text_centered(log.display()))
                    .on_press(Message::Open(log.display().to_string())),
                text_centered(format!("{} errors written", total)),
                text_centered(time),
                // .horizontal_alignment(Horizontal::Center),
                // space,
                row![continue_button]
                    .padding(4)
                    .align_items(Alignment::Center)
            ]
            .align_items(Alignment::Center)
            .padding(4)
            .spacing(6);

            container(view)
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
            let error_message = match errors.len() {
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

            let buttons = match manually_saved {
                true => row![continue_button],
                false => row![continue_button, save_errors_button],
            }
            .padding(4)
            .spacing(6)
            .align_items(Alignment::Center);

            let view = column![
                text("Done..."),
                text("But there's too many errors to display! (-_-')"),
                text("...and I can't store them to a file either:"),
                text_centered(format!("\"{}\"", reason)),
                // .style(style::text::Text::Error),
                buttons,
                error_message,
                discarded_errors,
            ]
            .align_items(Alignment::Center)
            .padding(4)
            .spacing(6);

            container(view)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        }
    }
}
