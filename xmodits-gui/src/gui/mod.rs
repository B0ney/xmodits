pub mod icons;
pub mod style;
pub mod views;
use crate::core::{
    cfg::{Config, SampleRippingConfig, GeneralConfig},
    font::JETBRAINS_MONO,
    xmodits::{xmodits_subscription, DownloadMessage},
};
use iced::{keyboard::{Event as KeyboardEvent, KeyCode}, widget::{text_input, scrollable, checkbox}};
use iced::widget::{button, column, container, row, text, Column, Container};
use iced::window::{Event as WindowEvent, Icon};
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
use xmodits_lib::exporter::AudioFormat;
// use views::settings::Message as SettingsMessage;
use views::trackers::Message as TrackerMessage;
use views::trackers::Trackers;
use iced::alignment::Horizontal;

#[derive(Default, Debug, Clone)]
pub enum View {
    #[default]
    Configure,
    // Settings,
    About,
    // Help,
}

#[derive(Debug, Clone)]
pub enum Message {
    ConfigurePressed,
    // SettingsPressed,
    AboutPressed,
    // HelpPressed,
    Tracker(TrackerMessage),
    SetCfg(ConfigMessage),
    SetRipCfg(ConfigRippingMessage),
    // ChangeSetting(SettingsMessage),
    About(AboutMessage),
    SetDestinationDialog,
    SaveConfig,
    StartRip,
    Progress(DownloadMessage),
    WindowEvent(Event),
    Ignore,
    Select {
        index: usize,
        selected: bool,
    },
    SelectAll(bool),
    DeleteSelected,
    Probe(usize),
    Open(PathBuf),

    AddFileDialog,
    AddFolderDialog,
    Clear,

    SetDestination(Option<PathBuf>),
    // SetFormat(AudioFormat),
    // SetNoFolderToggle(bool),
    // SetRecursionDepth(bool),
    // SetIndexOnly(bool),
    // SetIndexRaw(bool),
    // SetUpperCase(bool),
    // SetLowerCase(bool),
    // SetIndexPadding(bool),
    // SetPrefixSamples(bool),
}

#[derive(Default)]
pub struct XmoditsGui {
    view: View,
    config: Config,
    tracker: Trackers,
}

impl Application for XmoditsGui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let config = Config::load();
        (
            Self {
                tracker: Trackers::default(),
                config,
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("XMODITS")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ConfigurePressed => self.view = View::Configure,
            // Message::SettingsPressed => self.view = View::Settings,
            Message::AboutPressed => self.view = View::About,
            // Message::HelpPressed => self.view = View::Help,
            Message::Tracker(msg) => return self.tracker.update(msg).map(Message::Tracker),
            Message::SetCfg(msg) => self.config.ripping.naming.update(msg),
            Message::SetRipCfg(msg) => match msg {
                // ConfigRippingMessage::SetHint(hint) => {
                //     self.tracker.set_hint(hint.into());
                //     self.config.ripping.update(msg);
                // }
                _ => self.config.ripping.update(msg),
            },
            // Message::ChangeSetting(msg) => self.config.general.update(msg),
            Message::About(msg) => views::about::update(msg),
            Message::SetDestinationDialog => {
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .pick_folder()
                            .await
                            .map(|f| f.path().to_owned())
                    },
                    Message::SetDestination,
                )
            }
            Message::SetDestination(path) => {
                if let Some(destination) = path {
                    self.config.ripping.destination = destination;
                }
            }
            Message::StartRip => {
                self.tracker.start_rip(&self.config.ripping);
            }
            Message::Progress(msg) => {
                return self
                    .tracker
                    .update(TrackerMessage::SubscriptionMessage(msg))
                    .map(Message::Tracker)
            }
            Message::SaveConfig => {
                if let Err(e) = self.config.save() {
                    warn!("{}", e);
                };
            }
            Message::WindowEvent(e) => match e {
                Event::Keyboard(KeyboardEvent::KeyPressed { key_code, .. })
                    if key_code == KeyCode::Delete =>
                {
                    self.tracker.delete_selected()
                }

                Event::Window(WindowEvent::FileDropped(path)) => self.tracker.add(path),
                _ => (),
            },
            Message::Ignore => (),
            Message::Select { index, selected } => todo!(),
            Message::SelectAll(_) => todo!(),
            Message::DeleteSelected => todo!(),
            Message::Probe(_) => todo!(),
            Message::Open(_) => todo!(),
            Message::AddFileDialog => todo!(),
            Message::AddFolderDialog => todo!(),
            Message::Clear => todo!(),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message, Renderer<Self::Theme>> {
        let input = self
            .config
            .ripping
            .view_destination_bar()
            .map(Message::SetRipCfg);

        let set_destination: _ = row![
            input,
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
            // button("Settings")
            //     .on_press(Message::SettingsPressed)
            //     .padding(10),
            // button("Help").on_press(Message::HelpPressed).padding(10),
            button("About").on_press(Message::AboutPressed).padding(10),
        ]
        .spacing(5)
        .width(Length::FillPortion(1))
        .align_items(Alignment::Center);

        let g = match self.view {
            View::Configure => container(
                column![
                    self.tracker.view_current_tracker().map(|_| Message::Ignore),
                    self.config.ripping.naming.view().map(Message::SetCfg),
                    self.config.ripping.view().map(Message::SetRipCfg),
                    self.config
                        .ripping
                        .view_folder_scan_depth()
                        .map(Message::SetRipCfg),
                    // self.config.general.view().map(Message::ChangeSetting),
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
            // View::Settings => self.config.general.view().map(Message::ChangeSetting),
            View::About => views::about::view().map(Message::About),
            // View::Help => views::help::view().map(|_| Message::Ignore),
        };

        let main: _ = row![
            column![menu, g].spacing(10).width(Length::FillPortion(4)), // 8
            column![
                set_destination,
                self.tracker.view_trackers().map(Message::Tracker),
                Trackers::bottom_button().map(Message::Tracker),
            ]
            .width(Length::FillPortion(5)) //6
            .spacing(10),
        ]
        .spacing(10);

        let content = Column::new().spacing(15).height(Length::Fill).push(main);

        Container::new(content)
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

impl XmoditsGui {
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

        let _ = Self::run(settings);
    }
}

struct StartSignal;

#[derive(Default)]
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
        self.paths.iter().map(|f| f.selected).count()
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
    pub fn filename(&self) -> String {
        todo!()
    }
}

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
}
use iced::widget::Space;
use iced::widget::progress_bar;

#[derive(Default)]
pub struct App {
    view: View,
    state: State,
    general_config: GeneralConfig,
    ripping_config: SampleRippingConfig,
    entries: Entries,
    current: Option<Info>,
    sender: Option<Sender<StartSignal>>,
    // history: Vec<usize>,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self::default(),
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("XMODITS")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ConfigurePressed => self.view = View::Configure,
            Message::AboutPressed => self.view = View::About,
            Message::Tracker(_) => todo!(),
            Message::SetCfg(_) => todo!(),
            Message::SetRipCfg(_) => todo!(),
            Message::About(_) => todo!(),
            Message::SetDestinationDialog => todo!(),
            Message::SetDestination(_) => todo!(),
            Message::SaveConfig => todo!(),
            Message::StartRip => todo!(),
            Message::Progress(_) => todo!(),
            Message::WindowEvent(e) => match e {
                Event::Keyboard(KeyboardEvent::KeyPressed { key_code, .. }) => match key_code {
                    KeyCode::Delete => self.delete_selected(),
                    _ => (),
                }
                Event::Window(WindowEvent::FileDropped(path)) => self.entries.add(path),
                _ => (),
            },
            Message::Ignore => (),
            Message::SelectAll(selected) => self.entries.all_selected = selected,
            Message::DeleteSelected => self.delete_selected(),
            Message::Select { index, selected } => self.entries.select(index, selected),
            Message::Probe(_) => todo!(),
            Message::Open(_) => todo!(),
            Message::AddFileDialog => todo!(),
            Message::AddFolderDialog => todo!(),
            Message::Clear => todo!(),
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
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
                .spacing(10)
            ).into(),
            View::About => views::about::view().map(Message::About),
        };

        let left_half = column![
            menu, 
            left_half_view
        ]
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

        let main: _ = row![
            left_half,
            right_half            
        ]
        .spacing(10);

        let main: _ = Column::new()
            .spacing(15)
            .height(Length::Fill)
            .push(main);

        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .into()
    }
}

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

    pub fn delete_selected(&mut self) {
        // clear the entries if everything is selected
        if self.entries.all_selected || self.entries.total_selected() == self.entries.len() {
            self.entries.clear();
            self.current = None;
            return;
        }

        let mut i = 0;

        while i < self.entries.len() {
            let path = &self.entries.paths[i];
            if path.selected {
                if matches!(&self.current, Some(e) if e.matches(&path.path)) {
                    self.current = None;
                }
                let _ = self.entries.paths.remove(i);
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
            |s| Message::SetDestination(Some(PathBuf::new().join(s))),
        )
        .padding(10);

        input.into()
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
                        text(format!("Failed to load \"{}\"", path.display()))
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
                    container(text("Drag and Drop").font(JETBRAINS_MONO))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x()
                        .center_y()
                } else {
                    container(scrollable(self.entries.paths.iter().enumerate().fold(
                        column![].spacing(10).padding(5),
                        |s, (idx, gs)| {
                            s.push(row![
                                button(if gs.is_dir() {
                                    row![
                                        checkbox("", gs.selected, move |b| Message::Select{
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
                                        checkbox("", {
                                            // todo
                                            match self.entries.all_selected {
                                            true => true,
                                            false => gs.selected
                                        }
                                        }, move |b| Message::Select{
                                            index: idx,
                                            selected: b,
                                        }),
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
            },
            State::Ripping { ref message, progress } => container(
                column![
                    text(match message.as_ref() {
                        Some(info) => info,
                        None => "Ripping...",
                    })
                    .font(JETBRAINS_MONO),
                    progress_bar(0.0..=100.0, progress)
                        .height(5)
                        .width(200)
                ]
                .spacing(5)
                .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y(),
            State::Done => container(
                column![
                    text("Done! \\(^_^)/").font(JETBRAINS_MONO),
                    text("Drag and Drop").font(JETBRAINS_MONO)
                ]
                .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y(),
            State::DoneWithErrors { ref errors } => container(column![
                column![
                    text("Done... But Xmodits could not rip everything... (._.)")
                        .font(JETBRAINS_MONO)
                        .horizontal_alignment(Horizontal::Center)
                ]
                .padding(4),
                scrollable(
                    errors
                        .iter()
                        .fold(column![].spacing(10).padding(5), |t, (s, x)| {
                            t.push(row![
                                container(
                                    column![
                                        text("todo!()"), //filename
                                        text(x).horizontal_alignment(Horizontal::Center)
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
            State::DoneWithTooMuchErrors { ref log, ref errors } =>  container(
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
                    .style(style::button::Button::Hyperlink)
                ]
                .align_items(Alignment::Center)
                .padding(4)
                .spacing(5),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y(),
            State::DoneWithTooMuchErrorsNoLog { ref reason, ref errors } => todo!(),
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

fn icon() -> Icon {
    let image = image::load_from_memory(include_bytes!("../../res/img/logo/icon3.png")).unwrap();
    let (w, h) = image.dimensions();
    Icon::from_rgba(image.as_bytes().to_vec(), w, h).unwrap()
}
