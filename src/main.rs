#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // show logs when debugging
#![allow(dead_code)]

pub mod dialog;
pub mod font;
// pub mod icon;
pub mod logger;
pub mod sample_ripper;
pub mod screen;
pub mod theme;
pub mod utils;
pub mod widget;
// mod gui;
// mod simple;

use std::env;

use data::{config::SampleRippingConfig, Config};

use iced::{Application, Command, Settings, Subscription};
use iced::widget::column;
use screen::configuration::{sample_ripping, SampleConfigManager};

use sample_ripper::Message as SubscriptionMessage;

use iced::Event as IcedEvent;
use widget::Collection;
use screen::configuration::Message as ConfigMessage;
use crate::screen::build_info;

use widget::{Element, Column};

fn main() -> iced::Result {
    let args = env::args().skip(1);

    let version = args
        .peekable()
        .next()
        .map(|a| a == "--version" || a == "-V")
        .unwrap_or_default();

    if version {
        println!("XMODITS {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // let args: Vec<String> =  env::args().collect();

    XMODITS::launch()
}

/// XMODITS graphical application
#[derive(Debug, Default)]
pub struct XMODITS {
    config_manager: SampleConfigManager,
    
}

impl XMODITS {
    /// Launch the application
    pub fn launch() -> iced::Result {
        // Setup logging stuff
        logger::set_panic_hook();
        logger::init_logging();

        // TOOD Load fonts

        // load configuration
        let config = Config::load();

        //
        Self::run(Settings::default())
    }

    /// WINDOWS ONLY
    ///
    /// XMODITS' simple mode to allow dragging and dropping modules onto the binary
    #[cfg(windows)]
    pub fn launch_simple() {}

    pub fn settings() {}
    // fn load_config() {}
}

/// TODO: allow the user to customize their application icon
fn icon() -> iced::window::Icon {
    let icon_data = include_bytes!("../assets/img/logo/icon3.png");
    iced::window::icon::from_file_data(icon_data, None).unwrap()
}

#[derive(Debug, Clone)]
pub enum Message {
    Ignore,
    Config(ConfigMessage),
    FontsLoaded(Result<(), iced::font::Error>),
    #[cfg(feature = "audio")]
    AudioEngine(),
    Subscription(SubscriptionMessage),
    Iced(IcedEvent),
}

impl Application for XMODITS {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme; // TODO: replace with theme::Theme when implemented
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("XMDOITS - 10%") // todo: add status
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Ignore => (),
            Message::Config(msg) => return self.config_manager.update(msg).map(Message::Config),
            _ => (),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![].push_maybe(build_info::view())
        .push(self.config_manager.view_ripping_config().map(Message::Config))
            .push(self.config_manager.view_destination().map(Message::Config)).into()

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
