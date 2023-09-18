use crate::icon::GIF;
use super::{style, App, Info, Message, State};

use crate::core::xmodits::{CompleteState, StartSignal};
// use crate::font::JETBRAINS_MONO;
use crate::icon;
use crate::gui::utils::file_name;

use iced::alignment::Horizontal;
use iced::widget::{
    button, checkbox, column, container, progress_bar, row, scrollable, text, text_input, Space, lazy,
};
use iced::window::icon::from_rgba;
use iced::window::{Icon, Settings as Window};
use iced::{Alignment, Application, Command, Element, Length, Renderer, Settings};
use iced_gif::gif;
// use iced_lazy::lazy;

use image::{self, GenericImageView};
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use style::Theme;
use tracing::warn;


// See mod.rs for the full iced application
impl App {
    pub fn view_current_tracker(&self) -> Element<Message, Renderer<Theme>> {
        // let view_samples_button: _ = button("View Samples")
        //     .on_press(Message::Ignore)
        //     .padding(5);

        let content: _ = match &self.current {
            Some(info) => match info {
                Info::Valid {
                    name,
                    format,
                    samples,
                    total_sample_size,
                    ..
                } => container(
                    column![
                        text(format!("Module Name: {}", name.trim())),
                        text(format!("Format: {}", format)),
                        text(format!("Samples: {}", samples)),
                        text(format!("Total Sample Size: {} KiB", total_sample_size)),
                        // Space::with_width(15),
                        // view_samples_button,
                    ]
                    .spacing(5)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
                ),
                Info::Invalid { error, path } => container(
                    column![
                        text(format!("Failed to load \"{}\"", file_name(path)))
                            .horizontal_alignment(Horizontal::Center),
                        text(error).horizontal_alignment(Horizontal::Center),
                    ]
                    .spacing(5)
                    .align_items(Alignment::Center)
                    .width(Length::Fill),
                ),
            },
            None => container(text("None selected")
            // 
        ),
        };
        container(
            column![
                text("Current Tracker Information"),
                // ,
                content
                    .style(style::Container::Frame)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .padding(8)
                    .center_x()
                    .center_y()
            ]
            .spacing(15),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn view_entries(&self) -> Element<Message, Renderer<Theme>> {
        let total_modules: _ =
            text(format!("Entries: {}", self.entries.len()))
            // 
            ;

        let total_selected: _ =
            text(format!("Selected: {}", self.entries.total_selected()))
            // 
            ;

        let continue_button: _ = button("Continue")
            .on_press(Message::SetState(State::Idle))
            .padding(5);

        let save_errors_button: _ = button("Save Errors")
            .on_press(Message::SaveErrors)
            .padding(5);

        let cancel_ripping_button: _ = button("Cancel")
            .on_press(Message::Cancelled)
            .style(style::button::Button::Cancel)
            .padding(5);

        let invert_selection_button: _ = button("Invert")
            .on_press(Message::InvertSelection)
            .padding(5);

        let display: _ = match self.state {
            State::Idle => {
                if self.entries.is_empty() {
                    container(
                        column![text("Drag and Drop")
                        // 
                        , gif(&GIF.idle)]
                            .align_items(Alignment::Center),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                } else {
                    container(scrollable(lazy(&self.entries.entries, |_| {
                        column(
                            self.entries
                                .entries
                                .iter()
                                .enumerate()
                                .map(|(idx, entry)| {
                                    row![
                                        button({
                                            let item = row![
                                                checkbox("", entry.selected, move |b| {
                                                    Message::Select {
                                                        index: idx,
                                                        selected: b,
                                                    }
                                                }),
                                                text(&entry.filename()),
                                            ]
                                            .spacing(1)
                                            .align_items(Alignment::Center);

                                            if entry.is_dir() {
                                                item.push(Space::with_width(Length::Fill))
                                                    .push(icon::folder_icon())
                                            } else {
                                                item
                                            }
                                        })
                                        .width(Length::Fill)
                                        .on_press(Message::Probe(idx))
                                        .padding(4)
                                        .style(style::button::Button::Entry),
                                        Space::with_width(15)
                                    ]
                                    .into()
                                })
                                .collect(),
                        )
                        .spacing(10)
                        .padding(5)
                    })))
                    .height(Length::Fill)
                }
            }
            State::Done(ref completed_state) => match completed_state {
                CompleteState::SomeErrors(ref errors) => container(
                    column![
                        column![
                            text("Done... But xmodits could not rip everything... (._.)")
                                
                                .horizontal_alignment(Horizontal::Center),
                            text(&self.time),
                        ]
                        .padding(4)
                        .align_items(Alignment::Center),
                        row![continue_button, save_errors_button]
                            .padding(4)
                            .spacing(6)
                            .align_items(Alignment::Center),
                        scrollable(lazy((), |_| column(
                            errors
                                .iter()
                                .map(|failed| {
                                    row![
                                        container(
                                            column![
                                                text(failed.filename()),
                                                text(&failed.reason)
                                                    .horizontal_alignment(Horizontal::Center)
                                            ]
                                            .width(Length::Fill)
                                            .align_items(Alignment::Center)
                                        )
                                        .style(style::Container::Frame)
                                        .width(Length::Fill)
                                        .padding(4),
                                        Space::with_width(15)
                                    ]
                                    .into()
                                })
                                .collect()
                        )
                        .spacing(10)
                        .padding(5)
                        .width(Length::Fill)))
                        .height(Length::Fill),
                        // space,
                    ]
                    .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .height(Length::Fill),

            }
        };

        container(
            column![
                row![
                    total_modules,
                    total_selected,
                    Space::with_width(Length::Fill),
                    invert_selection_button,
                    // checkbox is 5 units taller than the other elements
                    checkbox("Select all", self.entries.all_selected, Message::SelectAll)
                        .style(style::checkbox::CheckBox::Inverted),
                ]
                .spacing(15)
                .align_items(Alignment::Center),
                display
                    .padding(5)
                    .style(style::Container::Black)
                    .width(Length::Fill),
            ]
            .spacing(10),
        )
        .height(Length::Fill)
        .into()
    }
}
