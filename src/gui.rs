#![allow(clippy::let_with_type_underscore)]
pub mod app;
// pub mod font;
// pub mod icons;
pub mod style;
pub mod utils;
pub mod views;

use data::config::{Config, GeneralConfig, SampleRippingConfig};
use data::entries::Entries;
use crate::core::xmodits::{
    xmodits_subscription, CompleteState, ErrorHandler, ExtractionMessage, Failed, StartSignal, CANCELLED,
};

use data::tracker_info::Info;
use iced::keyboard::{Event as KeyboardEvent, KeyCode};
use iced::widget::{button, column, container, row, text, Column, Container, Space};
use iced::window::Event as WindowEvent;
use iced::{Alignment, Application, Command, Element, Event, Length, Renderer, Subscription};

// use crate::font::JETBRAINS_MONO;
use style::Theme;
use utils::{create_file, files_dialog, folder_dialog, folders_dialog, tracker_info};

use views::about::Message as AboutMessage;
use views::config_general::Message as ConfigGeneralMessage;
use views::config_name::Message as ConfigNamingMessage;
use views::config_ripping::Message as ConfigRippingMessage;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tokio::sync::mpsc::Sender;
use xmodits_lib::traits::Module;

#[derive(Default, Debug, Clone)]
pub enum View {
    #[default]
    Configure,
    Settings,
    About,
    Help,
}

#[derive(Debug, Clone)]
pub enum Message {
    ConfigurePressed,
    SettingsPressed,
    AboutPressed,
    HelpPressed,
    SetNameCfg(ConfigNamingMessage),
    SetRipCfg(ConfigRippingMessage),
    SetGeneralCfg(ConfigGeneralMessage),
    // SetTheme(Theme),
    About(AboutMessage),
    SetDestinationDialog,
    SaveConfig,
    StartRip,
    Subscription(ExtractionMessage),
    WindowEvent(Event),
    Ignore,
    Select { index: usize, selected: bool },
    SelectAll(bool),
    DeleteSelected,
    Probe(usize),
    Open(PathBuf),
    AddFileDialog,
    AddFolderDialog,
    Clear,
    TrackerInfo(Option<Info>),
    Add(Option<Vec<PathBuf>>),
    SetDestination(Option<PathBuf>),
    SetState(State),
    SaveErrors,
    SaveErrorResult(Result<(), Vec<Failed>>),
    Cancelled,
    InvertSelection,
    FontsLoaded(Result<(), iced::font::Error>),
    // SaveFile(Option<PathBuf>),
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
    Done(CompleteState),
}

impl State {
    fn progress(&mut self, progress_update: f32, errors: u64) {
        if let Self::Ripping {
            progress,
            total_errors,
            ..
        } = self
        {
            *progress = progress_update;
            *total_errors = errors
        }
    }

    fn message(&mut self, message_update: Option<String>) {
        if let Self::Ripping { message, .. } = self {
            *message = message_update;
        }
    }
}

#[derive(Default)]
pub struct App {
    view: View,
    state: State,
    general_config: GeneralConfig,
    ripping_config: SampleRippingConfig,
    entries: Entries,
    current: Option<Info>,
    sender: Option<Sender<StartSignal>>,
    // history: History,
    time: data::time::Time,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let Config { ripping, general } = Config::load();
        (
            Self {
                ripping_config: ripping,
                general_config: general,
                ..Default::default()
            },
            Command::batch(vec![crate::font::load().map(Message::FontsLoaded)]),
        )
    }

    fn title(&self) -> String {
        String::from("XMODITS")
    }
    fn theme(&self) -> Self::Theme {
        self.general_config.theme.palette().clone()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ConfigurePressed => self.view = View::Configure,
            Message::AboutPressed => self.view = View::About,
            Message::HelpPressed => self.view = View::Help,
            Message::SettingsPressed => self.view = View::Settings,
            Message::SetNameCfg(msg) => self.ripping_config.naming.update(msg),
            Message::SetRipCfg(msg) => self.ripping_config.update(msg),
            Message::About(msg) => views::about::update(msg),
            Message::SetDestinationDialog => {
                return Command::perform(folder_dialog(), Message::SetDestination)
            }
            Message::SetDestination(destination) => {
                if let Some(s) = destination {
                    self.ripping_config.destination = s
                }
            }
            Message::SaveConfig => {
                let general = self.general_config.to_owned();
                let ripping = self.ripping_config.to_owned();

                return Command::perform(
                    async { Config { general, ripping }.save().await },
                    |_| Message::Ignore,
                );
            }
            Message::StartRip => return self.start_ripping(),
            Message::Subscription(m) => match m {
                ExtractionMessage::Ready(start_signal) => {
                    self.sender = Some(start_signal);
                }
                ExtractionMessage::Done(completed_state) => {
                    CANCELLED.store(false, std::sync::atomic::Ordering::Relaxed);
                    self.time.stop();
                    self.state = State::Done(completed_state)
                }
                ExtractionMessage::Progress {
                    progress,
                    total_errors,
                } => {
                    self.state.progress(progress, total_errors);
                }
                ExtractionMessage::Info(info) => self.state.message(info),
            },
            Message::WindowEvent(e) => match e {
                Event::Keyboard(KeyboardEvent::KeyPressed { key_code, .. }) => match key_code {
                    KeyCode::Delete => self.delete_selected(),
                    _ => (),
                },
                Event::Window(WindowEvent::FileDropped(path)) => self.add(path),
                _ => (),
            },
            Message::Ignore => (),
            Message::SelectAll(selected) => self.entries.select_all(selected),
            Message::DeleteSelected => self.delete_selected(),
            Message::Select { index, selected } => self.entries.select(index, selected),
            Message::Probe(index) => {
                let path = &self.entries.entries[index];

                if path.is_file() {
                    let command =
                        Command::perform(tracker_info(path.path.clone()), Message::TrackerInfo);

                    match self.current {
                        None => return command,
                        Some(ref info) if info.path() != path.path => {
                            return command;
                        }
                        _ => (),
                    }
                }
            }
            Message::Open(link) => {
                let _ = open::that(link);
            }
            Message::AddFileDialog => {
                return Command::perform(files_dialog(), Message::Add);
            }
            Message::AddFolderDialog => {
                return Command::perform(folders_dialog(), Message::Add);
            }
            Message::Clear => self.clear_entries(),
            Message::TrackerInfo(module) => {
                if module.is_some() {
                    self.current = module
                }
            }
            Message::Add(paths) => {
                if let Some(paths) = paths {
                    paths.into_iter().for_each(|path| self.add(path))
                }
            }
            Message::SetState(state) => self.state = state,
            // Message::SetTheme(theme) => self.general_config.theme = theme,
            Message::SaveErrors => {
                let State::Done(errors) =  &mut self.state else {
                    return Command::none();
                };
                let Some(errors) = errors.take() else {
                    return Command::none();
                };

                return Command::perform(
                    async {
                        let Some(path) = create_file().await else {
                            return Err(errors);
                        };

                        ErrorHandler::dump(errors, path).await
                    },
                    Message::SaveErrorResult,
                );
            }
            Message::SaveErrorResult(result) => {
                let State::Done(state) =  &mut self.state else {
                    return Command::none();
                };

                let Err(mut returned_errors) = result else {
                    // tell the user that they have successfully saved
                    // state.set_manually_saved();
                    self.state = State::Idle; // todo
                    return Command::none();
                };

                // Store the errors back
                if let Some(errors) = state.errors_ref_mut() {
                    *errors = std::mem::take(&mut returned_errors);
                };
            }
            Message::SetGeneralCfg(general_cfg) => self.general_config.update(general_cfg),
            Message::Cancelled => {
                self.state.message(Some("Cancelling...".into()));
                CANCELLED.store(true, std::sync::atomic::Ordering::Release)
            },
            Message::InvertSelection => self.entries.invert(),
            Message::FontsLoaded(_) => (),
        };
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Renderer<Self::Theme>> {
        let set_destination: _ = row![
            self.destination_bar(),
            button("Select")
                .on_press(Message::SetDestinationDialog)
                .padding(10),
        ]
        .spacing(5)
        .width(Length::FillPortion(1));

        let menu: _ = row![
            button("Configure")
                .on_press(Message::ConfigurePressed)
                .padding(10),
            button("Settings")
                .on_press(Message::SettingsPressed)
                .padding(10),
            // button("Help").on_press(Message::HelpPressed).padding(10),
            button("About").on_press(Message::AboutPressed).padding(10),
        ]
        .spacing(5)
        .width(Length::FillPortion(1))
        .align_items(Alignment::Center);

        
        let left_half_view: _ = match self.view {
            View::Configure => container(
                column![
                    self.view_current_tracker(),
                    self.ripping_config.naming.view(self.preview_sample_name()).map(Message::SetNameCfg),
                    self.ripping_config.view().map(Message::SetRipCfg),
                    row![
                        button("Save Configuration")
                            .padding(10)
                            .on_press(Message::SaveConfig),
                        button(
                            row![text("Start"), crate::icon::download_icon()]
                                .align_items(Alignment::Center)
                        )
                        .padding(10)
                        .on_press(Message::StartRip)
                        .style(style::button::Button::Start)
                        .width(Length::Fill),
                    ]
                    .spacing(5)
                    .width(Length::FillPortion(1))
                    .align_items(Alignment::Center),
                ]
                .spacing(10),
            )
            .into(),
            View::About => views::about::view().map(Message::About),
            View::Settings => self.general_config.view().map(Message::SetGeneralCfg),
            View::Help => views::help::view(),
        };

        let left_half = column![menu, left_half_view]
            .spacing(10)
            .width(Length::FillPortion(4));

        let mut right_half: _ = column![set_destination, self.view_entries()]
            .width(Length::FillPortion(5)) //6
            .spacing(10);

        const TOO_MUCH_FILES: usize = 200;

        if self.entries.files() > TOO_MUCH_FILES {
            let warning = format!("That's a lot of files! You REALLY should be using folders.");

            right_half = right_half.push(text(warning).style(style::text::Text::Error));
        }

        if !self.ripping_config.self_contained 
            && !self.ripping_config.naming.prefix 
        {
            let warning = format!("\"Self Contained\" is disabled. You should enable \"Prefix Samples\" to reduce collisions. Unless you know what you are doing.");

            right_half = right_half.push(text(warning).style(style::text::Text::Error));
        }

        right_half = right_half.push(
            row![
                button(text("Add File"))
                    .padding(10)
                    .on_press(Message::AddFileDialog),
                button(text("Add Folder"))
                    .padding(10)
                    .on_press(Message::AddFolderDialog),
                Space::with_width(Length::Fill),
                button("Delete Selected")
                    .padding(10)
                    .on_press(Message::DeleteSelected),
                button("Clear").padding(10).on_press(Message::Clear),
            ]
            .spacing(10),
        );

        let main: _ = row![left_half, right_half].spacing(10);

        let main: _ = Column::new().spacing(15).height(Length::Fill).push(main);

        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .into()
    }
    fn subscription(&self) -> Subscription<Message> {
        iced::Subscription::batch([
            iced::subscription::events().map(Message::WindowEvent),
            xmodits_subscription().map(Message::Subscription),
        ])
    }
}