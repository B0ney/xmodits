mod simple;

use std::path::PathBuf;

use crate::font::{self, JETBRAINS_MONO};
use crate::icon::{self, error};
use crate::logger;
use crate::ripper::subscription::CompleteState;
use crate::ripper::{self, Message as RipperMessage};
use crate::screen::config::advanced::AdvancedConfig;
use crate::screen::config::sample_naming::NamingConfig;
use crate::screen::config::sample_ripping::RippingConfig;
use crate::screen::config::{advanced, sample_naming};
use crate::screen::history::History;
use crate::screen::tracker_info::{self, TrackerInfo};
use crate::screen::{about, build_info, config::sample_ripping, main_panel::entry::Entries};
use crate::theme;
use crate::widget::{Collection, Column, Element};

// use data::time::Time;
use data::Config;

use iced::widget::column;
use iced::{window, Event as IcedEvent};
use iced::{Application, Command, Settings, Subscription};

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
    tracker_info: TrackerInfo,
    // sample_pack: (),
    theme: theme::Theme,
    naming_cfg: NamingConfig,
    ripping_cfg: RippingConfig,
    advanced_cfg: AdvancedConfig,
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
    // fn load_config() {}
}

/// TODO: allow the user to customize their application icon
fn icon() -> iced::window::Icon {
    iced::window::icon::from_file_data(include_bytes!("../assets/img/logo/icon2.png"), None)
        .unwrap()
}

pub fn settings(config: Config) -> iced::Settings<Config> {
    iced::Settings {
        default_font: JETBRAINS_MONO,
        default_text_size: 13.0.into(),
        exit_on_close_request: true,
        flags: config,
        window: window::Settings {
            icon: Some(icon()),
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
    AdvancedCfg(advanced::Message),
    #[cfg(feature = "audio")]
    Audio(),

    ConfigPressed,
    DeleteSelected,
    FontsLoaded(Result<(), iced::font::Error>),
    Iced(IcedEvent),
    Ignore,
    InvertSelection,
    NamingCfg(sample_naming::Message),
    Probe(usize),
    ProbeResult(TrackerInfo),
    RippingCfg(sample_ripping::Message),
    SaveConfig,
    Subscription(RipperMessage),
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
                if let Some(paths) = paths {
                    self.entries.add_multiple(paths)
                }
            }
            Message::AdvancedCfg(msg) => self.advanced_cfg.update(msg),
            Message::ConfigPressed => self.view = View::Configure,
            Message::DeleteSelected => self.entries.delete_selected(),
            Message::FontsLoaded(result) => {
                if result.is_err() {
                    tracing::error!("could not load font")
                }
            }
            Message::Iced(_) => {}
            Message::Ignore => (),
            Message::RippingCfg(msg) => {
                return self.ripping_cfg.update(msg).map(Message::RippingCfg)
            }
            Message::InvertSelection => self.entries.invert(),
            Message::NamingCfg(msg) => self.naming_cfg.update(msg),
            Message::Probe(idx) => {
                let path = self.entries.get(idx).unwrap().to_owned();
                return Command::perform(tracker_info::probe(path), Message::ProbeResult);
            }
            Message::ProbeResult(probe) => self.tracker_info = probe,
            Message::SaveConfig => {}
            Message::Subscription(msg) => match msg {
                RipperMessage::Ready(sender) => self.ripper.set_sender(sender),
                RipperMessage::Info(info) => self.state.update_message(info),
                RipperMessage::Progress { progress, errors } => {
                    self.state.update_progress(progress, errors)
                }
                RipperMessage::Done { state, time } => {
                    self.ripper.reset_stop_flag(); // todo: should this be here?
                    self.state = State::Finished { state, time }
                }
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let right_half = ();
        let left_half = ();

        column![]
            .push_maybe(build_info::view())
            .push(self.ripping_cfg.view().map(Message::RippingCfg))
            .push(self.naming_cfg.view().map(Message::NamingCfg))
            .push(sample_ripping::view_destination_bar(&self.ripping_cfg).map(Message::RippingCfg))
            .push(icon::download())
            .push(icon::play())
            .push(icon::pause())
            .push(about::view().map(Message::About))
            .into()

        // self.config_manager.view_destination().map(Message::Config).into()
        // todo!()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::Subscription::batch([
            iced::event::listen().map(Message::Iced),
            ripper::xmodits_subscription().map(Message::Subscription),
        ])
    }
}
