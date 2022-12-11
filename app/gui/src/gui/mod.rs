pub mod icons;
pub mod style;
pub mod views;
use crate::core;
use crate::core::cfg::Config;
use crate::core::font::JETBRAINS_MONO;
use crate::core::xmodits::{self, xmodits_subscription};
use iced::keyboard::{Event as KeyboardEvent, KeyCode};
use iced::widget::{
    button, checkbox, column, container, pick_list, row, scrollable, text, text_input, Button,
    Checkbox, Column, Container, Row, Space, Text,
};
use iced::window::Event as WindowEvent;
use iced::window::Icon;
use iced::{time, Alignment, Event, Subscription};
use iced::{window::Settings as Window, Application, Command, Element, Length, Renderer, Settings};
use image::{self, GenericImageView};
use std::path::PathBuf;
use style::Theme;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::{info, warn};

use views::about::Message as AboutMessage;
use views::config_name::Message as ConfigMessage;
use views::config_ripping::Message as ConfigRippingMessage;
use views::settings::Message as SettingsMessage;
use views::trackers::Message as TrackerMessage;
use views::trackers::Trackers;

use xmodits::DownloadMessage;

#[derive(Default, Debug, Clone)]
pub enum View {
    #[default]
    Configure,
    Settings,
    About,
    Help,
    Ripping,
}

#[derive(Debug, Clone)]
pub enum Message {
    ConfigurePressed,
    SettingsPressed,
    AboutPressed,
    HelpPressed,

    Tracker(TrackerMessage),
    SetCfg(ConfigMessage),
    SetRipCfg(ConfigRippingMessage),
    ChangeSetting(SettingsMessage),
    About(AboutMessage),

    SetDestinationDialog,
    SetDestination(Option<PathBuf>),

    Beep(String),
    SaveConfig,
    StartRip,

    Progress(xmodits::DownloadMessage),
    WindowEvent(Event),

    Ignore,
}

#[derive(Default)]
pub struct XmoditsGui {
    view: View,
    config: Config,
    audio: core::sfx::Audio,
    tracker: Trackers,
    sender: Option<Sender<xmodits::gh>>,
}

impl Application for XmoditsGui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                config: Config::load(),
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
            Message::SettingsPressed => self.view = View::Settings,
            Message::AboutPressed => self.view = View::About,
            Message::HelpPressed => self.view = View::Help,
            Message::Tracker(msg) => return self.tracker.update(msg).map(Message::Tracker),
            Message::SetCfg(msg) => self.config.name_cfg_mut().update(msg),
            Message::SetRipCfg(msg) => self.config.ripping.update(msg),
            Message::ChangeSetting(msg) => self.config.general.update(msg),
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
            Message::Beep(sfx) => self.audio.play(&sfx),
            Message::StartRip => {
                if let Some(ref mut tx) = self.sender {
                    let _ =
                        tx.try_send((self.tracker.move_paths(), self.config.ripping.to_owned()));
                }
            }
            Message::Progress(msg) => match msg {
                DownloadMessage::Ready(tx) => self.sender = Some(tx),
                DownloadMessage::Done => {
                    info!("Done!"); // notify when finished ripping
                }
                DownloadMessage::Progress { progress, result } => {
                    info!("{}", progress);

                    if let Err((path, e)) = result {
                        warn!("{} <-- {}", &path.display(), e);
                        self.audio.play("sfx_2")
                    }
                } // useful for progress bars
            },
            Message::SaveConfig => {
                // TODO: wrap config save in command, make a new async save method.
                let _ = self.config.save();
                self.audio.play("sfx_1");
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
        }
        Command::none()
    }

    fn view(&self) -> Element<Message, Renderer<Self::Theme>> {
        let trackers: _ = self.tracker.view_trackers().map(Message::Tracker);

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
            button("Settings")
                .on_press(Message::SettingsPressed)
                .padding(10),
            button("About").on_press(Message::AboutPressed).padding(10),
            button("Help").on_press(Message::HelpPressed).padding(10),
        ]
        .spacing(5)
        .width(Length::FillPortion(1))
        .align_items(Alignment::Center);

        let g = match self.view {
            View::Configure => container(
                column![
                    self.tracker.view_current_tracker().map(|_| Message::Ignore),
                    self.config.name_cfg().view().map(Message::SetCfg),
                    self.config.ripping.view().map(Message::SetRipCfg),
                    row![button("Save Config")
                        .padding(10)
                        .on_press(Message::SaveConfig)]
                    .width(Length::FillPortion(1))
                    .align_items(Alignment::End),
                    button("Start")
                        .padding(10)
                        .on_press(Message::StartRip)
                        .width(Length::Fill),
                ]
                .spacing(10),
            )
            .into(),
            View::Settings => self.config.general.view().map(Message::ChangeSetting),
            View::About => views::about::view().map(Message::About),
            View::Help => views::help::view().map(|_| Message::Ignore),
            View::Ripping => todo!(),
        };

        let main: _ = row![
            column![menu, g].spacing(10).width(Length::FillPortion(4)), // 8
            column![set_destination, trackers]
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
                size: (700, 450),
                resizable: true,
                decorations: true,
                icon: Some(icon()),
                ..iced::window::Settings::default()
            },
            // try_opengles_first: true,
            default_text_size: 17,
            ..iced::Settings::default()
        };

        Self::run(settings).unwrap_err();
    }
}

fn icon() -> Icon {
    let image =
        image::load_from_memory(include_bytes!("../../../../extras/logos/png/icon3.png")).unwrap();
    let (w, h) = image.dimensions();
    Icon::from_rgba(image.as_bytes().to_vec(), w, h).unwrap()
}
