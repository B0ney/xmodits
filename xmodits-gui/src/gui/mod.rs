#![allow(clippy::let_with_type_underscore)]
pub mod app;
pub mod font;
pub mod icons;
pub mod style;
pub mod utils;
pub mod views;

use crate::core::cfg::{Config, GeneralConfig, SampleRippingConfig};
use crate::core::entries::{Entries, History};
use crate::core::xmodits::{xmodits_subscription, CompleteState, ExtractionMessage, StartSignal};

use iced::keyboard::{Event as KeyboardEvent, KeyCode};
use iced::widget::{button, column, container, row, text, Column, Container, Space};
use iced::window::Event as WindowEvent;
use iced::{Alignment, Application, Command, Element, Event, Length, Renderer, Subscription};

use font::JETBRAINS_MONO;
use style::Theme;
use utils::{files_dialog, folder_dialog, folders_dialog, tracker_info};
use views::about::Message as AboutMessage;
use views::config_name::Message as ConfigMessage;
use views::config_ripping::Message as ConfigRippingMessage;
// use views::settings::Message as SettingsMessage;

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
    SetCfg(ConfigMessage),
    SetRipCfg(ConfigRippingMessage),
    SetTheme(Theme),
    About(AboutMessage),
    SetDestinationDialog,
    SaveConfig,
    StartRip { rip_selected: bool },
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
    SetState(State)
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

#[derive(Debug, Clone)]
pub enum Info {
    Valid {
        path: PathBuf,
        name: String,
        format: String,
        samples: usize,
        total_sample_size: usize,
    },
    Invalid {
        path: PathBuf,
        error: String,
    },
}

impl Info {
    pub fn matches(&self, other: &Path) -> bool {
        matches!(
            self,
            Self::Invalid { path, .. } |
            Self::Valid { path, ..} if path == other
        )
    }
    pub fn path(&self) -> &Path {
        match self {
            Self::Invalid { path, .. } | Self::Valid { path, .. } => path,
        }
    }
    pub fn valid(tracker: Box<dyn Module>, path: PathBuf) -> Self {
        Self::Valid {
            name: tracker.name().to_owned(),
            format: tracker.format().to_owned(),
            samples: tracker.total_samples(),
            path,
            total_sample_size: tracker
                .samples()
                .iter()
                .map(|f| f.length as usize)
                .sum::<usize>()
                / 1024,
        }
    }
    pub fn invalid(error: String, path: PathBuf) -> Self {
        Self::Invalid { error, path }
    }
}

struct Time {
    start: Instant,
    duration: Duration,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            duration: Default::default(),
        }
    }
}

impl Time {
    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    pub fn stop(&mut self) {
        self.duration = self.start.elapsed();
    }

    pub fn elapsed(&self) -> f32 {
        self.duration.as_secs_f32()
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.duration.as_secs_f32() {
            s if s < 60.0 => format!("{} second(s)", s),
            s if s < 60.0 * 60.0 => format!("{} minute(s)", s / 60.0),
            s => format!("{} hour(s)", s / (60.0 * 60.0)),
        };
        write!(f, "Took {}", s)
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
    history: History,
    time: Time,
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
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("XMODITS")
    }
    fn theme(&self) -> Self::Theme {
        self.general_config.theme.clone()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ConfigurePressed => self.view = View::Configure,
            Message::AboutPressed => self.view = View::About,
            Message::HelpPressed => self.view = View::Help,
            Message::SettingsPressed => self.view = View::Settings,
            Message::SetCfg(msg) => self.ripping_config.naming.update(msg),
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
            Message::StartRip { rip_selected } => self.start_ripping(rip_selected),
            Message::Subscription(m) => match m {
                ExtractionMessage::Ready(start_signal) => {
                    self.sender = Some(start_signal);
                }
                ExtractionMessage::Done(completed_state) => {
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
            Message::SetTheme(theme) => self.general_config.theme = theme,
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
            button("Help").on_press(Message::HelpPressed).padding(10),
            button("About").on_press(Message::AboutPressed).padding(10),
        ]
        .spacing(5)
        .width(Length::FillPortion(1))
        .align_items(Alignment::Center);

        let left_half_view: _ = match self.view {
            View::Configure => container(
                column![
                    self.view_current_tracker(),
                    self.ripping_config.naming.view().map(Message::SetCfg),
                    self.ripping_config.view().map(Message::SetRipCfg),
                    self.ripping_config
                        .view_folder_scan_depth()
                        .map(Message::SetRipCfg),
                    row![
                        button("Save Configuration")
                            .padding(10)
                            .on_press(Message::SaveConfig),
                        button(
                            row![text("Start"), icons::download_icon()]
                                .align_items(Alignment::Center)
                        )
                        .padding(10)
                        .on_press(Message::StartRip { rip_selected: false })
                        .style(style::button::Button::Start)
                        .width(Length::Fill),
                        // button(
                        //     row![text("Start (Selected)"), icons::download_icon()]
                        //         .align_items(Alignment::Center)
                        // )
                        // .padding(10)
                        // .on_press(Message::StartRip { rip_selected: true })
                        // .style(style::button::Button::Start)
                        // .width(Length::Fill),
                    ]
                    .spacing(5)
                    .width(Length::FillPortion(1))
                    .align_items(Alignment::Center),
                ]
                .spacing(10),
            )
            .into(),
            View::About => views::about::view().map(Message::About),
            View::Settings => self.view_settings(),
            View::Help => views::help::view(),
        };

        let left_half = column![menu, left_half_view]
            .spacing(10)
            .width(Length::FillPortion(4));

        let right_half: _ = column![
            set_destination,
            self.view_entries(),
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
            .spacing(10)
        ]
        .width(Length::FillPortion(5)) //6
        .spacing(10);

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
