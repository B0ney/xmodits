pub mod icons;
pub mod style;
pub mod views;
use crate::core::{
    cfg::Config,
    font::JETBRAINS_MONO,
    xmodits::{xmodits_subscription, DownloadMessage},
};
use iced::keyboard::{Event as KeyboardEvent, KeyCode};
use iced::widget::{button, column, container, row, text, Column, Container};
use iced::window::{Event as WindowEvent, Icon};
use iced::{
    window::Settings as Window, Alignment, Application, Command, Element, Event, Length, Renderer,
    Settings, Subscription,
};
use image::{self, GenericImageView};
use std::path::PathBuf;
use style::Theme;
use tracing::warn;
use views::about::Message as AboutMessage;
use views::config_name::Message as ConfigMessage;
use views::config_ripping::Message as ConfigRippingMessage;
// use views::settings::Message as SettingsMessage;
use views::trackers::Message as TrackerMessage;
use views::trackers::Trackers;

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
    SetDestination(Option<PathBuf>),
    SaveConfig,
    StartRip,
    Progress(DownloadMessage),
    WindowEvent(Event),
    Ignore,
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
                tracker: Trackers::default().with_hint(config.ripping.hint.into()),
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
                ConfigRippingMessage::SetHint(hint) => {
                    self.tracker.set_hint(hint.into());
                    self.config.ripping.update(msg);
                }
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
            default_text_size: 17,
            ..iced::Settings::default()
        };

        let _ = Self::run(settings);
    }
}

fn icon() -> Icon {
    let image = image::load_from_memory(include_bytes!("../../res/img/logo/icon3.png")).unwrap();
    let (w, h) = image.dimensions();
    Icon::from_rgba(image.as_bytes().to_vec(), w, h).unwrap()
}
