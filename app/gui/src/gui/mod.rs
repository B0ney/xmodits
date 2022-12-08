pub mod icons;
pub mod style;
pub mod views;
pub mod widgets;
use crate::core::dialog;
use crate::core::cfg::Config;
use crate::core::font::JETBRAINS_MONO;
use crate::core::{
    self,
    xmodits::{self},
};
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
use rfd::AsyncFileDialog;
use std::path::PathBuf;
use std::time::Duration;
use style::Theme;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::info;
use views::about::{AboutView, Message as AboutMessage};
use views::configure::{ConfigView, Message as ConfigMessage};
use views::settings::{Message as SettingsMessage, SettingsView};
use views::trackers::{Message as TrackerMessage, Xmodits};

use crate::core::xmodits::build_subscription;
fn icon() -> Icon {
    let image =
        image::load_from_memory(include_bytes!("../../../../extras/logos/png/icon3.png")).unwrap();
    let (w, h) = image.dimensions();
    Icon::from_rgba(image.as_bytes().to_vec(), w, h).unwrap()
}

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
    Rip,
    Tracker(TrackerMessage),
    SetCfg(ConfigMessage),
    ChangeSetting(SettingsMessage),
    About(AboutMessage),
    Beep(String),
    StartRip,
    OpenFileDialoge,
    AddFile(Option<PathBuf>),
    WindowEvent(Event),
    ClearTrackers,
    DeleteSelected,
    _None,
    Progress(xmodits::DownloadMessage),
}

#[derive(Default)]
pub struct XmoditsGui {
    view: View,
    cfg: ConfigView,
    settings: SettingsView,
    about: AboutView,
    audio: core::sfx::Audio,
    // ripper: core::xmodits::Ripper,
    tracker: Xmodits,
    sender: Option<Sender<xmodits::DownloadMessage>>,
}

impl Application for XmoditsGui {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("XMODITS")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Rip => todo!(),
            Message::SetCfg(cfg) => {
                if self.cfg.update(cfg) {
                    self.audio.play("sfx_2")
                }
            }
            Message::Beep(sfx) => self.audio.play(&sfx),
            Message::StartRip => match self.sender {
                Some(ref mut tx) => {
                    tx.try_send(xmodits::DownloadMessage::Download(
                        (
                            self.tracker.cloned_paths(),
                            self.cfg.cfg.to_owned()
                        )
                    ));
                }
                _ => (),
            },

            Message::OpenFileDialoge => {
                return Command::perform(
                    async {
                        // tokio::
                        match rfd::FileDialog::new().pick_file() {
                            Some(handle) => Some(handle),
                            None => None,
                        }
                    },
                    Message::AddFile,
                )
            }
            Message::AddFile(path) => {
                if let Some(path) = path {
                    self.tracker.update(TrackerMessage::Add(path));
                    self.audio.play("sfx_1");
                }
            }
            Message::ConfigurePressed => self.view = View::Configure,
            Message::SettingsPressed => self.view = View::Settings,
            Message::AboutPressed => self.view = View::About,
            Message::HelpPressed => self.view = View::Help,
            Message::ChangeSetting(msg) => match msg {
                SettingsMessage::SFX(sfx) => self.audio.play(&sfx),
                _ => self.settings.update(msg),
            },
            Message::_None => (),
            Message::WindowEvent(e) => match e {
                Event::Keyboard(k) => match k {
                    KeyboardEvent::KeyPressed { key_code, .. } if key_code == KeyCode::Delete => {
                        self.tracker.update(TrackerMessage::DeleteSelected);
                    }
                    _ => (),
                },
                Event::Window(f) => match f {
                    WindowEvent::FileDropped(path) => {
                        self.tracker.update(TrackerMessage::Add(path));
                    }

                    _ => (),
                },
                _ => (),
            },
            Message::ClearTrackers => return self.tracker.update(TrackerMessage::Clear).map(Message::Tracker),
            Message::Tracker(msg) => return self.tracker.update(msg).map(Message::Tracker),
            Message::DeleteSelected => return self.tracker.update(TrackerMessage::DeleteSelected).map(Message::Tracker),
            Message::About(msg) => self.about.update(msg),
            Message::Progress(msg) => match msg {
                xmodits::DownloadMessage::Sender(tx) => self.sender = Some(tx),
                xmodits::DownloadMessage::Download(_) => (),
                xmodits::DownloadMessage::Cancel => (),
                xmodits::DownloadMessage::Done => self.audio.play("sfx_1"),
                xmodits::DownloadMessage::Progress => self.audio.play("sfx_1"),
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message, Renderer<Self::Theme>> {
        let trackers: _ = self.tracker.view_trackers().map(Message::Tracker);

        let buttonx = row![
            button("Add").padding(10).on_press(Message::OpenFileDialoge),
            button("Add Folder")
                .padding(10)
                .on_press(Message::OpenFileDialoge),
            Space::with_width(Length::Fill),
            button(row![icons::delete_icon(), "Delete Selected"])
                .padding(10)
                .on_press(Message::DeleteSelected),
            button("Clear").padding(10).on_press(Message::ClearTrackers),
            // button("Clear").padding(10).on_press(Message::ClearTrackers),
            // Space::with_width(Length::Fill),
        ]
        .spacing(10);

        let trackers = column![trackers, Space::with_width(Length::Units(5)), buttonx]
            .width(Length::FillPortion(1))
            .spacing(5);

        use ConfigMessage::*;
        let destination = format!("{}",&self.cfg.cfg.destination.display());
        let input: _ = text_input("Output Directory", &destination, |s| {
            Message::SetCfg(DestinationFolder(PathBuf::new().join(s)))
        })
        .padding(10)
        .on_submit(Message::Beep("sfx_1".into()));

        let set_destination: _ = row![
            input,
            button("Select")
                .on_press(Message::Beep("sfx_1".into()))
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
                    self.tracker.view_current_tracker().map(|_| Message::_None),
                    self.cfg.view().map(Message::SetCfg),
                    button("Start")
                        .padding(10)
                        .on_press(Message::StartRip)
                        .width(Length::Fill),
                ]
                .spacing(10),
            )
            .into(),
            View::Settings => self.settings.view().map(Message::ChangeSetting),
            View::About => self.about.view().map(Message::About),
            _ => container(text(":(")).into(),
        };

        let main: _ = row![
            column![menu, g,].spacing(10).width(Length::FillPortion(4)), // 8
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
            build_subscription().map(Message::Progress)
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
