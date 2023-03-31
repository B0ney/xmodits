pub mod app;
pub mod icons;
pub mod style;
mod utils;
pub mod views;
pub mod font;
use font::JETBRAINS_MONO;
use crate::core::{
    cfg::{Config, GeneralConfig, SampleRippingConfig},
    log::async_write_error_log,
    xmodits::{xmodits_subscription, DownloadMessage, StartSignal},
};
use chrono::Utc;
use iced::widget::{button, column, container, row, text, Column, Container};
use iced::window::{Event as WindowEvent, Icon};
use iced::{
    keyboard::{Event as KeyboardEvent, KeyCode},
    widget::{checkbox, scrollable, text_input},
};
use iced::{
    window::Settings as Window, Alignment, Application, Command, Element, Event, Length, Renderer,
    Settings, Subscription,
};
use image::{self, GenericImageView};
use std::path::{Path, PathBuf};
use style::Theme;
use tokio::sync::mpsc::Sender;
use tracing::warn;
use views::about::Message as AboutMessage;
use views::config_name::Message as ConfigMessage;
use views::config_ripping::Message as ConfigRippingMessage;
use xmodits_lib::{exporter::AudioFormat, traits::Module};
// use views::settings::Message as SettingsMessage;
use iced::alignment::Horizontal;

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
    SetState(State),
    // ChangeSetting(SettingsMessage),
    About(AboutMessage),
    SetDestinationDialog,
    SaveConfig,
    StartRip,
    Progress(DownloadMessage),
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
}

#[derive(Default, Debug, Clone)]
pub enum State {
    #[default]
    Idle,
    Ripping {
        message: Option<String>,
        progress: f32,
    },
    Done,
    DoneWithErrors {
        errors: Vec<(PathBuf, String)>,
    },
    DoneWithTooMuchErrors {
        log: PathBuf,
        errors: Vec<(PathBuf, String)>,
    },
    DoneWithTooMuchErrorsNoLog {
        reason: String,
        errors: Vec<(PathBuf, String)>,
    },
}

impl State {
    fn progress(&mut self, progress_update: f32) {
        match self {
            Self::Ripping { progress, .. } => {
                *progress = progress_update;
            }
            _ => (),
        }
    }

    fn message(&mut self, message_update: Option<String>) {
        match self {
            Self::Ripping { message, .. } => {
                *message = message_update;
            }
            _ => (),
        }
    }
}

#[derive(Default)]
pub struct History {
    history_entry: HistoryEntry
}

#[derive(Default)]
pub struct HistoryEntry {
    timestamp: chrono::DateTime<Utc>,
    entries: Entries,
}

#[derive(Default)]
pub struct Entries {
    pub all_selected: bool,
    pub paths: Vec<Entry>,
}

impl Entries {
    pub fn contains(&self, path: &Path) -> bool {
        self.paths.iter().any(|x| x.path == path)
    }

    pub fn add(&mut self, path: PathBuf) {
        self.paths.push(Entry {
            path,
            selected: false,
        })
    }

    pub fn total_selected(&self) -> usize {
        self.paths.iter().filter(|f| f.selected).count()
    }

    pub fn clear(&mut self) {
        self.all_selected = false;
        self.paths.clear();
    }

    pub fn len(&self) -> usize {
        self.paths.len()
    }

    pub fn select(&mut self, index: usize, selected: bool) {
        if let Some(entry) = self.paths.get_mut(index) {
            entry.selected = selected;
        }
    }
}

#[derive(Default)]
pub struct Entry {
    pub path: PathBuf,
    pub selected: bool,
}

impl Entry {
    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    pub fn filename(&self) -> String {
        self.path
            .file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or_default()
            .into()
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
use iced::widget::progress_bar;
use iced::widget::Space;

use self::utils::{files_dialog, folder_dialog, folders_dialog, tracker_info};

#[derive(Default)]
pub struct App {
    view: View,
    state: State,
    general_config: GeneralConfig,
    ripping_config: SampleRippingConfig,
    entries: Entries,
    current: Option<Info>,
    sender: Option<Sender<StartSignal>>,
    errors: Vec<(PathBuf, String)>, // for now
    history: History, 
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
            Message::SaveConfig => todo!(),
            Message::StartRip => self.start_ripping(),
            Message::Progress(m) => match m {
                DownloadMessage::Ready(start_signal) => {
                    self.sender = Some(start_signal);
                }
                DownloadMessage::Done => {
                    let errors = std::mem::take(&mut self.errors);
                    let log_path = self.ripping_config.destination.to_owned();

                    const MAX_ERRORS: usize = 150;

                    return Command::perform(
                        async move {
                            match errors.len() {
                                0 => State::Done,
                                1..=MAX_ERRORS => State::DoneWithErrors { errors },
                                _ => {
                                    // display the first 100 errors

                                    // we then output the errors to the file
                                    let mut errors_to_write = errors;
                                    let some_errs = errors_to_write.drain(..30).collect();

                                    match async_write_error_log(log_path, errors_to_write).await {
                                        Ok(log) => State::DoneWithTooMuchErrors {
                                            log,
                                            errors: some_errs,
                                        },
                                        Err(e) => State::DoneWithTooMuchErrorsNoLog {
                                            reason: e.to_string(),
                                            errors: some_errs,
                                        },
                                    }
                                }
                            }
                        },
                        Message::SetState,
                    );
                }
                DownloadMessage::Progress { progress, result } => {
                    self.state.progress(progress);

                    if let Err(error) = result {
                        self.errors.push(error);
                    }
                }
                DownloadMessage::Info(info) => self.state.message(info),
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
            Message::SelectAll(selected) => self.entries.all_selected = selected,
            Message::DeleteSelected => self.delete_selected(),
            Message::Select { index, selected } => self.entries.select(index, selected),
            Message::Probe(index) => {
                let path = &self.entries.paths[index];

                if path.is_file() {
                    let command =
                        Command::perform(tracker_info(path.path.to_owned()), Message::TrackerInfo);

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
                // todo: is this blocking?
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
            button("Help")
                .on_press(Message::HelpPressed)
                .padding(10),
            button("About")
                .on_press(Message::AboutPressed)
                .padding(10),
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
            View::Settings => views::settings::view(),
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
                // .style(style::button::Button::Delete),
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
            xmodits_subscription().map(Message::Progress),
        ])
    }
}