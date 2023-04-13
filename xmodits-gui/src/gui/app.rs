use super::icons::GIF;
use super::{style, App, Info, Message, State};

use crate::core::entries::{Entries, History};
use crate::core::xmodits::{CompleteState, StartSignal};
use crate::gui::font::JETBRAINS_MONO;
use crate::gui::icons;
use crate::gui::utils::file_name;

use iced::alignment::Horizontal;
use iced::widget::{
    button, checkbox, column, container, progress_bar, row, scrollable, text, text_input, Space,
};
use iced::window::icon::from_rgba;
use iced::window::{Icon, Settings as Window};
use iced::{Alignment, Application, Element, Length, Renderer, Settings};
use iced_gif::gif;

use image::{self, GenericImageView};
use std::path::PathBuf;
use style::Theme;

fn icon() -> Icon {
    let image = image::load_from_memory(include_bytes!("../../res/img/logo/icon3.png")).unwrap();
    let (w, h) = image.dimensions();
    
    from_rgba(image.as_bytes().to_vec(), w, h).unwrap()
}

// See mod.rs for the full iced application
impl App {
    pub fn start() {
        let settings: Settings<()> = Settings {
            window: Window {
                size: (780, 640),
                resizable: true,
                decorations: true,
                icon: Some(icon()),
                ..iced::window::Settings::default()
            },
            // try_opengles_first: true,
            default_text_size: 17.0,
            ..iced::Settings::default()
        };
        GIF.init_lazy();
        let _ = Self::run(settings);
    }

    pub fn add(&mut self, path: PathBuf) {
        // If the application is currently ripping, ignore
        if matches!(self.state, State::Ripping { .. }) {
            return;
        }
        // Only add the path if it doesn't exist
        if !self.entries.contains(&path) {
            self.entries.add(path)
        }
        // Set the state to idle if not...
        self.state = State::Idle;
    }

    pub fn clear_entries(&mut self) {
        self.entries.clear();
        self.current = None;

        if !matches!(self.state, State::Idle | State::Ripping { .. }) {
            self.state = State::Idle;
        };
    }

    pub fn delete_selected(&mut self) {
        // clear the entries if everything is selected
        if self.entries.all_selected || self.entries.total_selected() == self.entries.len() {
            self.entries.clear();
            self.current = None;
            return;
        }

        let mut i = 0;

        while i < self.entries.len() {
            let path = &self.entries.entries[i];
            if path.selected {
                if matches!(&self.current, Some(e) if e.matches(&path.path)) {
                    self.current = None;
                }
                let _ = self.entries.entries.remove(i);
            } else {
                i += 1;
            }
        }
        self.entries.all_selected = false;
    }

    pub fn destination_bar(&self) -> Element<Message, Renderer<Theme>> {
        let destination = &self.ripping_config.destination;
        let input: _ = text_input(
            "Output Directory",
            &format!("{}", destination.display()),
        )
        .padding(10)
        .on_input(|s| Message::SetDestination(Some(PathBuf::new().join(s))));

        input.into()
    }

    pub fn start_ripping(&mut self) {
        if self.entries.len() == 0 {
            return;
        }

        let Some(sender) = &self.sender.to_owned() else {
            return;
        };

        let _ = sender.try_send(self.bulid_start_signal());
        self.time.start();

        self.state = State::Ripping {
            message: None,
            progress: 0.0,
            total_errors: 0,
        }
    }

    fn bulid_start_signal(&mut self) -> StartSignal {
        let ripping_config = self.ripping_config.to_owned();
        self.current = None;

        let paths: Vec<PathBuf> = std::mem::take(&mut self.entries.entries)
            .into_iter()
            .map(|f| f.path)
            .collect();

        (paths, ripping_config)
    }

    pub fn view_current_tracker(&self) -> Element<Message, Renderer<Theme>> {
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
                        text(format!("Module Name: {}", name)),
                        text(format!("Format: {}", format)),
                        text(format!("Samples: {}", samples)),
                        text(format!("Total Sample Size: {} KiB", total_sample_size)),
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
            None => container(text("None selected").font(JETBRAINS_MONO)),
        };
        container(
            column![
                text("Current Tracker Information").font(JETBRAINS_MONO),
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
            text(format!("Modules: {}", self.entries.len())).font(JETBRAINS_MONO);

        let total_selected: _ =
            text(format!("Selected: {}", self.entries.total_selected())).font(JETBRAINS_MONO);

        let display: _ = match self.state {
            State::Idle => {
                if self.entries.len() == 0 {
                    container(column![
                        text("Drag and Drop").font(JETBRAINS_MONO),
                        // gif(&GIF.idle)
                    ].align_items(Alignment::Center))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                } else {
                    container(scrollable(self.entries.entries.iter().enumerate().fold(
                        column![].spacing(10).padding(5),
                        |s, (idx, gs)| {
                            s.push(row![
                                button(if gs.is_dir() {
                                    row![
                                        checkbox("", gs.selected, move |b| Message::Select {
                                            index: idx,
                                            selected: b,
                                        }),
                                        text(&gs.filename()),
                                        Space::with_width(Length::Fill),
                                        icons::folder_icon()
                                    ]
                                    .spacing(1)
                                    .align_items(Alignment::Center)
                                } else {
                                    row![
                                        checkbox(
                                            "",
                                            match self.entries.all_selected {
                                                true => true,
                                                false => gs.selected,
                                            },
                                            move |b| Message::Select {
                                                index: idx,
                                                selected: b,
                                            }
                                        ),
                                        text(&gs.filename()),
                                    ]
                                    .spacing(1)
                                    .align_items(Alignment::Center)
                                })
                                .width(Length::Fill)
                                .on_press(Message::Probe(idx))
                                .padding(4)
                                .style(style::button::Button::Entry),
                                Space::with_width(15)
                            ])
                        },
                    )))
                    .height(Length::Fill)
                }
            }
            State::Ripping {
                ref message,
                progress,
                ..
            } => container(
                column![
                    text(match message.as_ref() {
                        Some(info) => info,
                        None => "Ripping...",
                    })
                    .font(JETBRAINS_MONO).horizontal_alignment(Horizontal::Center),
                    progress_bar(0.0..=100.0, progress).height(5).width(200),
                    // gif(&GIF.ripping)
                ]
                .spacing(8)
                .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y(),

            State::Done(ref completed_state) => match completed_state {
                CompleteState::NoErrors => container(
                    column![
                        text("Done! \\(^_^)/").font(JETBRAINS_MONO),                      
                        text("Drag and Drop").font(JETBRAINS_MONO),
                        text(&self.time).font(JETBRAINS_MONO),
                    ]
                    .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y(),
                CompleteState::SomeErrors(ref errors) => container(column![
                    column![
                        text("Done... But xmodits could not rip everything... (._.)")
                            .font(JETBRAINS_MONO)
                            .horizontal_alignment(Horizontal::Center),
                        text(&self.time)
                            .font(JETBRAINS_MONO)
                    ]
                    .padding(4)
                    .align_items(Alignment::Center),
                    scrollable(
                        errors
                            .iter()
                            .fold(column![].spacing(10).padding(5), |t, failed| {
                                t.push(row![
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
                                ])
                            })
                            .width(Length::Fill),
                    ),
                ])
                .width(Length::Fill)
                .height(Length::Fill),

                CompleteState::TooMuchErrors { log, total } => container(
                    column![
                        text("Done...").font(JETBRAINS_MONO),
                        text("But there's too many errors to display! (-_-')").font(JETBRAINS_MONO),
                        text("Check the logs at:").font(JETBRAINS_MONO),
                        button(
                            text(log.display())
                                .font(JETBRAINS_MONO)
                                .horizontal_alignment(Horizontal::Center)
                        )
                        .padding(0)
                        .on_press(Message::Open(log.to_owned()))
                        .style(style::button::Button::HyperlinkInverted),
                        text(format!("{} errors written", total))
                            .font(JETBRAINS_MONO)
                            .horizontal_alignment(Horizontal::Center),
                        text(&self.time)
                            .font(JETBRAINS_MONO)
                            .horizontal_alignment(Horizontal::Center),
                    ]
                    .align_items(Alignment::Center)
                    .padding(4)
                    .spacing(6),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y(),

                CompleteState::TooMuchErrorsNoLog {
                    ref reason,
                    ref errors,
                    discarded,
                } => container(
                    column![
                        text("Done...").font(JETBRAINS_MONO),
                        text("But there's too many errors to display! (-_-')").font(JETBRAINS_MONO),
                        text("...and I can't store them to a file either:").font(JETBRAINS_MONO),
                        text(format!("\"{}\"", reason)).font(JETBRAINS_MONO),
                        text(format!("{} stored errors", errors.len()))
                            .font(JETBRAINS_MONO)
                            .horizontal_alignment(Horizontal::Center),
                        text(match discarded {
                            0 => format!("No errors were discarded."),
                            n => format!("{} error(s) was discarded to save memory. >_<", n),
                        })
                        .font(JETBRAINS_MONO)
                        .horizontal_alignment(Horizontal::Center),
                        text(&self.time)
                            .font(JETBRAINS_MONO)
                            .horizontal_alignment(Horizontal::Center),
                    ]
                    .align_items(Alignment::Center)
                    .padding(4)
                    .spacing(6),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y(),
            },
        };

        container(
            column![
                row![
                    total_modules,
                    total_selected,
                    Space::with_width(Length::Fill),
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
