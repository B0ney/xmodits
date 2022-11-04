pub mod style;
pub mod widgets;
pub mod views;

use std::path::PathBuf;
use std::time::Duration;
use crate::core;
use crate::core::cfg::Config;
use crate::core::font::JETBRAINS_MONO;
use iced::{Alignment, Subscription, time, Event};
use iced::widget::{container,Space,column, Container, Column, checkbox,Checkbox, pick_list, Row, Text, button, Button, row, scrollable, text_input, text};
use iced::window::Icon;
use iced::{window::Settings as Window, Application, Command, Element, Length, Renderer, Settings};
use image::{self, GenericImageView};
use iced_native::window::Event as WindowEvent;

use rfd::AsyncFileDialog;

use views::configure::{Message as ConfigMessage, ConfigView};
use views::settings::{Message as SettingsMessage, SettingsView};
use self::views::about::AboutView;    
use style::Theme;


// const copypasta: &str = r#"Is your son obsessed with "Lunix"? BSD, Lunix, Debian and Mandrake are all versions of an illegal hacker operation system, invented by a Soviet computer hacker named Linyos Torovoltos, before the Russians lost the Cold War. It is based on a program called " xenix", which was written by Microsoft for the US government. These programs are used by hackers to break into other people's computer systems to steal credit card numbers. They may also be used to break into people's stereos to steal their music, using the "mp3" program. Torovoltos is a notorious hacker, responsible for writing many hacker programs, such as "telnet", which is used by hackers to connect to machines on the internet without using a telephone. Your son may try to install " lunix" on your hard drive. If he is careful, you may not notice its presence, however, lunix is a capricious beast, and if handled incorrectly, your son may damage your computer, and even break it completely by deleting Windows, at which point you will have to have your computer repaired by a professional."#;


fn icon() -> Icon {
    let image = image::load_from_memory(include_bytes!("../../../../extras/logos/png/icon3.png")).unwrap();
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
    check(bool),
    SetCfg(ConfigMessage),
    ChangeSetting(SettingsMessage),
    Beep(String),
    StartRip,
    OpenFileDialoge,
    AddFile(Option<PathBuf>),
    WindowEvent(Event),
    ClearTrackers,
    _None,
}

#[derive(Default)]
pub struct XmoditsGui {
    view: View,
    // cfg: core::cfg::Config,
    cfg: ConfigView,
    settings: SettingsView,
    about: AboutView,
    paths: Vec<String>,
    toggls: bool,
    audio: core::sfx::Audio,
    ripper: core::xmodits::Ripper,
    // ripper: TestOne
}

impl Application for XmoditsGui {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        // let c = ["it", "xm", "s3m", "mod", "umx"];
        // let a = (1000..1200).into_iter().map(|d| format!("{}.{}",d.to_string(), c[d % c.len()])).collect();
        // println!("{:?}",&a);
        (
            Self{
                // paths: a,
                // cfg: Config::load(),
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
            Message::Rip => todo!(),
            Message::check(g) => self.toggls = g,
            Message::SetCfg(cfg) => {
                if self.cfg.update(cfg) {
                    self.audio.play("sfx_2")
                }
            },
            Message::Beep(sfx) =>  self.audio.play(&sfx) ,
            Message::StartRip => return Command::perform(
                async {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    String::from("sfx_1")
                },Message::Beep
            ),
            Message::OpenFileDialoge => return Command::perform(
                async {
                    // tokio::
                    match rfd::FileDialog::new()
                        .pick_file(){
                            Some(handle) => Some(handle),
                            None => None
                        }
                }, Message::AddFile
            ),
            Message::AddFile(path) => {
                if let Some(path) = path {
                    self.paths.push(format!("{}", path.display()));
                    self.audio.play("sfx_1");
                }
            }
            Message::ConfigurePressed => self.view = View::Configure,
            Message::SettingsPressed => self.view = View::Settings,
            Message::AboutPressed => self.view = View::About,
            Message::HelpPressed => self.view = View::Help,
            Message::ChangeSetting(msg) => self.settings.update(msg),
            Message::_None => (),
            Message::WindowEvent(e) => match e {
                Event::Window(f) => match f {
                    WindowEvent::FileDropped(path) => {
                        self.paths.push(format!("{}", path.display()));
                        // self.audio.play("sfx_1");
                    },
                    _ => ()
                },
                _ => ()
            },
            Message::ClearTrackers => self.paths.clear(),
        }
        Command::none()
    }

    fn view(&self) -> Element<Message, Renderer<Self::Theme>> {
        let total_modules: _ =  text(format!("Total Modules: {}", self.paths.len()));
        let trackers: _ = self
            .paths
            .iter()
            .fold(
                Column::new().spacing(10).padding(5),
                |s, gs| s.push(row![
                    button(text(&gs))
                        .style(style::button::Button::NormalPackage)
                        .on_press(Message::Beep("sfx_1".into()))
                        .width(Length::Fill),
                    Space::with_width(Length::Units(15))
                ])
            );

        let buttonx = row![
            button("Add Module").padding(10).on_press(Message::OpenFileDialoge),
            Space::with_width(Length::Fill),
            button("Clear All").padding(10).on_press(Message::ClearTrackers),
            Space::with_width(Length::Fill),
            
            button("Start Ripping").padding(10).on_press(Message::Beep("sfx_1".into())),
        ].spacing(10);

        let trackers = column![
            total_modules,
            container(
                scrollable(trackers).height(Length::Fill)
            ).padding(5)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Container::Black),
            buttonx
        ]
        .width(Length::FillPortion(1))
        .spacing(5)
        .align_items(Alignment::Center);

        use ConfigMessage::*;

        let input: _ = text_input(
            "Destination", &self.cfg.cfg.destination, |s| Message::SetCfg(DestinationFolder(s))
        ).padding(10).on_submit(Message::Beep("sfx_1".into()));

        let set_destination: _ = row![
            // Space::with_width(Length::Units(5)),
            button("Open")
                .on_press(Message::Beep("sfx_1".into()))
                .padding(10),
            input,
            // Space::with_width(Length::Units(15)),
                // .style(style::button::Button::Refresh)
            
        ]
        .spacing(5)
        .width(Length::FillPortion(1));
        let logo:_ = text("0.0.7-Alpha").font(JETBRAINS_MONO);

        let menu: _ = row![
            logo,
            Space::with_width(Length::Fill),
            button("Configure")
                .on_press(Message::ConfigurePressed)
                .padding(10),
            button("Settings")
                .on_press(Message::SettingsPressed)
                .padding(10),
            button("About")
                .on_press(Message::AboutPressed)
                .padding(10),
            button("Help")
                .on_press(Message::HelpPressed)
                .padding(10),            
        ]
        .spacing(5)
        .width(Length::FillPortion(1)).align_items(Alignment::Center);

        // let top_panel: _ = row![
        //     // title,
        //     set_destination,
        //     // menu
        // ]
        // .width(Length::Fill)
        // .spacing(5);
        
        let stats: _ =  column![
            text("Current Tracker Infomation:").font(JETBRAINS_MONO),
            container(
                scrollable(
                    column![text("Module Name: NYC Streets"),
                        text("Format: Impulse Tracker"),
                        text("Samples: 26"),
                        text("Approx Total Sample Size (KiB): 1532"),
                        text("Comments: \n"),
                    ]
                    .spacing(5)
                    .width(Length::Fill)
                )
                .height(Length::Fill)
                .style(style::scrollable::Scrollable::Dark)
            )
            .style(style::Container::Frame)
            .padding(8)
            .width(Length::Fill)
            .height(Length::Fill)
        ]
        .spacing(5);

        let g = match self.view {
            View::Configure => {
                container(
                    column![
                        self.cfg.view().map(Message::SetCfg),
                        stats
                    ].spacing(8)
                    
                ).into()
            },
            View::Settings => {
                self.settings.view().map(Message::ChangeSetting)
            },
            View::About => {
                self.about.view().map(|_| Message::_None)
            }
            _ => container(stats).into(),
            // View::Settings => todo!(),
            // View::About => todo!(),
            // View::Help => todo!(),
            // View::Ripping => todo!(),
        };

        let main: _ = row![
            column![
                set_destination,
                trackers
            ]
            .width(Length::FillPortion(5))
            .spacing(10),
            column![
                menu,
                // top_buttons,
                g,
                 
            ]
            .width(Length::FillPortion(6))
            .spacing(10)
            // .width(Length::FillPortion(1))
            // .spacing(10)

        ].spacing(10);


        let content = Column::new()
            .spacing(15)
            .height(Length::Fill)
            // .push(top_panel)
            .push(main);
        
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            // .center_x()
            // .center_y()
            .into()
    }
    fn subscription(&self) -> Subscription<Message> {
        // time::Duration::from_secs(1).map(Msg::Beep)
        iced::subscription::events().map(Message::WindowEvent)
    }

}

impl XmoditsGui {
    pub fn start() {
        let settings: Settings<()> = Settings {
            window: Window {
                size: (900, 600),
                resizable: true,
                decorations: true,
                icon: Some(icon()),
                ..iced::window::Settings::default()
            },
            default_text_size: 17,
            ..iced::Settings::default()
        };

        Self::run(settings).unwrap_err();
    }
}


// #[derive(Default)]
// struct TestOne;

// impl TestOne {
//     // pub fn rip
//     pub fn subscription(&self) -> Subscription<Msg> {

//     }
// }

// async fn rips(
//     state: RipState
// ) -> (Option<(RipProgress)>, RipState){
//     match state {
//         RipState::start(time) => {
//             async {
//                 std::thread::sleep(Duration::from_secs(1));
//                 (Some(RipProgress::Advanced(1)), RipState::Ripping)
//             }
//         },
//         RipState::Finished => todo!(),
//     }
// }

// #[derive(Debug, Clone)]
// pub enum RipProgress {
//     Failed(usize),
//     Advanced(usize),
//     Finished,
// }

// enum RipState {
//     start(usize),
//     Ripping,
//     Finished,
// }