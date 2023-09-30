mod simple;

use std::path::PathBuf;

use crate::event;
use crate::font;
use crate::icon;
use crate::logger;
use crate::ripper::{self, subscription::CompleteState};
use crate::screen::config::name_preview::{self, SampleNameParams};
use crate::screen::config::sample_naming::{self, NamingConfig};
use crate::screen::config::sample_ripping::{self, RippingConfig, DESTINATION_BAR_ID};
use crate::screen::history::History;
use crate::screen::main_panel;
use crate::screen::tracker_info::{self, TrackerInfo};
use crate::screen::{about, build_info, main_panel::entry::Entries};
use crate::theme;
use crate::utils::{files_dialog, folders_dialog};
use crate::widget::{helpers, Collection, Column, Container, Element};
use helpers::{action, warning};

use data::Config;

use iced::widget::{button, checkbox, column, row, text_input, Space};
use iced::{window, Application, Command, Length, Subscription};

/// XMODITS graphical application
#[derive(Default)]
pub struct XMODITS {
    entries: Entries,
    history: History,
    state: State,
    view: View,
    ripper: ripper::Handle,
    #[cfg(feature = "audio")]
    audio: audio_engine::Handle,
    tracker_info: Option<TrackerInfo>,
    // sample_pack: (),
    theme: theme::Theme,
    naming_cfg: NamingConfig,
    sample_name: SampleNameParams,
    ripping_cfg: RippingConfig,
}

impl XMODITS {
    /// Launch the application
    pub fn launch() -> iced::Result {
        // Setup logging stuff
        logger::set_panic_hook();
        logger::init_logging();

        // load configuration
        let config = Config::load();

        //
        tracing::info!("Launcing GUI");
        Self::run(settings(config))
    }

    /// WINDOWS ONLY
    ///
    /// XMODITS' simple mode to allow dragging and dropping modules onto the binary
    #[cfg(windows)]
    pub fn launch_simple() {
        simple::rip(std::env::args())
    }

    pub fn settings() {}

    pub fn build_start_signal(&mut self) -> ripper::Signal {
        let entries = self.entries.take();
        let ripping = self.ripping_cfg.0.to_owned();
        let naming = self.naming_cfg.0.to_owned();

        ripper::Signal {
            entries,
            ripping,
            naming,
        }
    }
}

/// TODO: allow the user to customize their application icon
fn icon() -> iced::window::Icon {
    let icon = include_bytes!("../assets/img/logo/icon2.png");
    iced::window::icon::from_file_data(icon, None).unwrap()
}

pub fn settings(config: Config) -> iced::Settings<Config> {
    iced::Settings {
        default_font: font::JETBRAINS_MONO,
        default_text_size: 13.0.into(),
        exit_on_close_request: true,
        flags: config,
        window: window::Settings {
            icon: Some(icon()),
            min_size: Some((800, 720)),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// The current state of the application.
#[derive(Default, Debug, Clone)]
pub enum State {
    #[default]
    Idle,
    /// The user is previewing some samples
    SamplePreview(/* TODO */),
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
    },
}

impl State {
    fn update_progress(&mut self, new_progress: f32, new_errors: u64) {
        if let Self::Ripping {
            progress, errors, ..
        } = self
        {
            *progress = new_progress;
            *errors = new_errors;
        }
    }

    fn update_message(&mut self, new_message: Option<String>) {
        if let Self::Ripping { message, .. } = self {
            *message = new_message
        }
    }

    fn set_message(&mut self, message: impl Into<String>) {
        self.update_message(Some(message.into()))
    }

    fn is_ripping(&self) -> bool {
        matches!(self, Self::Ripping { .. })
    }
}

/// TODO: rename to avoid confusion
///
/// This is basically the configuration panel view.
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
    About(about::Message),
    AboutPressed,
    Add(Option<Vec<PathBuf>>),
    #[cfg(feature = "audio")]
    Audio(),
    Cancel,
    Clear,
    ConfigPressed,
    DeleteSelected,
    Event(event::Event),
    FileDialog,
    FolderDialog,
    FontsLoaded(Result<(), iced::font::Error>),
    Ignore,
    InvertSelection,
    NamingCfg(sample_naming::Message),
    Open(String),
    Probe(usize),
    ProbeResult(TrackerInfo),
    RippingCfg(sample_ripping::Message),
    SaveConfig,
    SaveConfigResult(),
    SaveErrors,
    SaveErrorsResult(),
    Select {
        index: usize,
        selected: bool,
    },
    SelectAll(bool),
    SetTheme,
    StartRipping,
    Subscription(ripper::Message),
}

impl Application for XMODITS {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = Config;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), font::load().map(Message::FontsLoaded))
    }

    fn title(&self) -> String {
        String::from("XMDOITS - 10%") // todo: add status
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::About(msg) => about::update(msg),
            Message::AboutPressed => self.view = View::About,
            Message::Add(paths) => {
                if self.state.is_ripping() {
                    return Command::none();
                }

                if let Some(paths) = paths {
                    self.entries.add_multiple(paths)
                }
                // todo: change state to idlde?
            }
            Message::Cancel => {
                self.state.set_message("Cancelling...");
                self.ripper.cancel();
            }
            Message::Clear => {
                self.tracker_info = None;
                self.entries.clear();
            }
            Message::ConfigPressed => self.view = View::Configure,
            Message::DeleteSelected => {
                // todo: clear tracker info if the selected path is equal
                self.entries.delete_selected()
            }
            Message::Event(event) => match event {
                event::Event::Clear => self.entries.clear(),
                event::Event::CloseRequested => {}
                event::Event::Delete => self.entries.delete_selected(),
                event::Event::FileDropped(file) => self.entries.add(file),
                event::Event::Save => {}
                event::Event::Start => {}
            },
            Message::FileDialog => return Command::perform(files_dialog(), Message::Add),
            Message::FolderDialog => return Command::perform(folders_dialog(), Message::Add),
            Message::FontsLoaded(result) => {
                if result.is_err() {
                    tracing::error!("could not load font")
                }
            }
            Message::Ignore => (),
            Message::RippingCfg(msg) => {
                return self.ripping_cfg.update(msg).map(Message::RippingCfg)
            }
            Message::InvertSelection => self.entries.invert(),
            Message::NamingCfg(msg) => self.naming_cfg.update(msg),
            Message::Open(link) => {
                if let Err(err) = open::that_detached(link) {
                    tracing::warn!("Could not open external link: {:?}", err)
                };
            }
            Message::Probe(idx) => {
                let path = self.entries.get(idx).unwrap().to_owned();
                return Command::perform(tracker_info::probe(path), Message::ProbeResult);
            }
            Message::ProbeResult(probe) => self.tracker_info = Some(probe),
            Message::SaveConfig => {}
            Message::SaveConfigResult() => {}
            Message::SaveErrors => todo!(),
            Message::SaveErrorsResult() => todo!(),
            Message::Select { index, selected } => self.entries.select(index, selected),
            Message::SelectAll(selected) => self.entries.select_all(selected),
            Message::SetTheme => todo!(),
            Message::StartRipping => {
                if self.state.is_ripping() | self.entries.is_empty() | !self.ripper.is_active() {
                    return Command::none();
                }

                if !self.ripping_cfg.destination_is_valid() {
                    tracing::error!(
                        "The provided destination is not valid. The *parent* folder must exist."
                    );
                    return text_input::focus(DESTINATION_BAR_ID.clone());
                }

                let start_signal = self.build_start_signal();
                let _ = self.ripper.send(start_signal);

                self.state = State::Ripping {
                    message: None,
                    progress: 0.0,
                    errors: 0,
                }
            }
            Message::Subscription(msg) => match msg {
                ripper::Message::Ready(sender) => self.ripper.set_sender(sender),
                ripper::Message::Info(info) => self.state.update_message(info),
                ripper::Message::Progress { progress, errors } => {
                    self.state.update_progress(progress, errors)
                }
                ripper::Message::Done { state, time } => {
                    self.ripper.reset_stop_flag(); // todo: should this be here?
                    self.state = State::Finished { state, time };
                    tracing::debug!("{:#?}", self.state);
                }
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let top_left_menu = row![
            button("Configure").on_press(Message::ConfigPressed),
            button("Settings").on_press(Message::ConfigPressed),
            button("About").on_press(Message::AboutPressed),
        ];

        let is_ripping = self.state.is_ripping();

        let bottom_left_buttons = row![
            button("Save Configuration").on_press(Message::SaveConfig),
            action(
                row!["START", icon::download()],
                Message::StartRipping,
                || !is_ripping
            )
        ];

        let left_view = match self.view {
            View::Configure => column![
                self.naming_cfg.view().map(Message::NamingCfg),
                self.ripping_cfg.view().map(Message::RippingCfg),
                bottom_left_buttons,
            ]
            .into(),
            View::Settings => todo!(),
            View::About => about::view().map(Message::About),
            View::Help => todo!(),
        };

        let left_half = column![top_left_menu, left_view];

        let destination =
            sample_ripping::view_destination_bar(&self.ripping_cfg).map(Message::RippingCfg);

        let right_bottom_buttons = row![
            action("Add File", Message::FileDialog, || !is_ripping),
            action("Add Folder", Message::FolderDialog, || !is_ripping),
            Space::with_width(Length::Fill),
            action("Delete Selected", Message::DeleteSelected, || !is_ripping),
            action("Clear", Message::Clear, || !is_ripping),
        ];

        let top_right_buttons = row![
            button("Invert")
                .on_press(Message::InvertSelection)
                .padding(5),
            checkbox("Select All", self.entries.all_selected, Message::SelectAll)
        ];

        let main_view = match &self.state {
            State::Idle => main_panel::view_entries(&self.entries),
            State::SamplePreview() => todo!(),
            State::Ripping {
                message,
                progress,
                errors,
            } => main_panel::view_ripping(message, *progress, *errors),
            State::Finished { state, time } => main_panel::view_finished(state, time),
        };

        let bad_cfg_warning = warning(
            || !self.ripping_cfg.0.self_contained && !self.naming_cfg.0.prefix,
            "\"Self Contained\" is disabled. You should enable \"Prefix Samples\" to reduce collisions. Unless you know what you are doing."
        );

        let too_many_files_warning = warning(
            || self.entries.len() > 200,
            "That's a lot of files! You REALLY should be using folders.",
        );

        let right_half = column![destination, top_right_buttons, main_view]
            .push_maybe(bad_cfg_warning)
            .push_maybe(too_many_files_warning)
            .push(right_bottom_buttons);

        let main = row![left_half, right_half];

        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::Subscription::batch([
            event::events().map(Message::Event),
            ripper::xmodits_subscription().map(Message::Subscription),
        ])
    }
}
