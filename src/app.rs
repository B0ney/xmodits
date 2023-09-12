mod simple;

use crate::font::{self, JETBRAINS_MONO};
use crate::icon;
use crate::logger;
use crate::sample_ripper::handle::SubscriptionHandle;
use crate::sample_ripper::subscription::CompleteState;
use crate::sample_ripper::{self, Message as SubscriptionMessage};
use crate::screen::{
    about::{self, Message as AboutMessage},
    build_info,
    configuration::{sample_ripping, Message as ConfigMessage, SampleConfigManager},
    main_panel::entry::Entries,
};
use crate::theme;
use crate::widget::{Collection, Column, Element};

use data::time::Time;
use data::{config::SampleRippingConfig, Config};

use iced::widget::column;
use iced::Event as IcedEvent;
use iced::{Application, Command, Settings, Subscription};

/// XMODITS graphical application
#[derive(Default)]
pub struct XMODITS {
    config_manager: SampleConfigManager,
    entries: Entries,
    state: State,
    handle: SubscriptionHandle,
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
        Self::run(settings())
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
    iced::window::icon::from_file_data(include_bytes!("../assets/img/logo/icon3.png"), None)
        .unwrap()
}

pub fn settings() -> iced::Settings<()> {
    iced::Settings {
        default_font: JETBRAINS_MONO,
        default_text_size: 13.0,
        ..Default::default()
    }
}

/// The current state of the application.
#[derive(Default, Debug, Clone)]
pub enum State {
    #[default]
    Idle,
    /// The user is previewing some samples
    SamplePreview(
        /* TODO */
    ),
    /// The application is currently ripping samples
    Ripping {
        message: Option<String>,
        progress: f32,
        total_errors: u64,
    },
    /// The application has finished ripping samples
    Finished {
        state: CompleteState,
        time: Time,
    },
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
    About(AboutMessage),
    AboutPressed,

    #[cfg(feature = "audio")]
    Audio(),

    Config(ConfigMessage),
    FontsLoaded(Result<(), iced::font::Error>),
    Iced(IcedEvent),
    Ignore,
    Subscription(SubscriptionMessage),
}

impl Application for XMODITS {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), font::load().map(Message::FontsLoaded))
    }

    fn title(&self) -> String {
        String::from("XMDOITS - 10%") // todo: add status
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Ignore => (),
            Message::Config(msg) => return self.config_manager.update(msg).map(Message::Config),
            Message::About(msg) => about::update(msg),
            _ => (),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let right_half = ();
        let left_half = ();

        column![]
            .push_maybe(build_info::view())
            .push(
                self.config_manager
                    .view_ripping_config()
                    .map(Message::Config),
            )
            .push(
                self.config_manager
                    .view_naming_config()
                    .map(Message::Config),
            )
            .push(self.config_manager.view_destination().map(Message::Config))
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
            sample_ripper::xmodits_subscription().map(Message::Subscription),
        ])
    }
}
