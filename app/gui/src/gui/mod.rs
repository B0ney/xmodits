pub mod style;

use std::path::PathBuf;
use std::time::Duration;
use crate::core;
use crate::core::cfg::Config;
use crate::core::font::JETBRAINS_MONO;
use iced::{Alignment, Subscription, time};
use iced::widget::{container,Space,column, Container, Column, checkbox,Checkbox, pick_list, Row, Text, button, Button, row, scrollable, text_input, text};
use iced::window::Icon;
use iced::{window::Settings as Window, Application, Command, Element, Length, Renderer, Settings};
use image::{self, GenericImageView};
use rfd::AsyncFileDialog;

use style::Theme;
const copypasta: &str = r#"Is your son obsessed with "Lunix"? BSD, Lunix, Debian and Mandrake are all versions of an illegal hacker operation system, invented by a Soviet computer hacker named Linyos Torovoltos, before the Russians lost the Cold War. It is based on a program called " xenix", which was written by Microsoft for the US government. These programs are used by hackers to break into other people's computer systems to steal credit card numbers. They may also be used to break into people's stereos to steal their music, using the "mp3" program. Torovoltos is a notorious hacker, responsible for writing many hacker programs, such as "telnet", which is used by hackers to connect to machines on the internet without using a telephone. Your son may try to install " lunix" on your hard drive. If he is careful, you may not notice its presence, however, lunix is a capricious beast, and if handled incorrectly, your son may damage your computer, and even break it completely by deleting Windows, at which point you will have to have your computer repaired by a professional."#;


fn icon() -> Icon {
    let image = image::load_from_memory(include_bytes!("../../../../extras/logos/png/icon3.png")).unwrap();
    let (w, h) = image.dimensions();
    Icon::from_rgba(image.as_bytes().to_vec(), w, h).unwrap()
}

#[derive(Debug, Clone)]
pub enum Msg {
    Rip,
    check(bool),
    SetCfg(CfgMsg),
    Beep(String),
    StartRip,
    OpenFileDialoge,
    AddFile(Option<PathBuf>),
}

#[derive(Debug, Clone)]
enum CfgMsg {
    NoFolder(bool),
    IndexOnly(bool),
    IndexRaw(bool),
    UpperCase(bool),
    LowerCase(bool),
    IndexPadding(usize),
    DestinationFolder(String),
}

fn set_cfg(cfg: &mut core::cfg::Config, msg: CfgMsg) -> bool {
    match msg {
        CfgMsg::NoFolder(b) => cfg.no_folder = b,
        CfgMsg::IndexOnly(b) => {
            if b {
                cfg.upper = false;
                cfg.lower = false;
            }
            cfg.index_only = b;
        },
        CfgMsg::IndexRaw(b) => cfg.index_raw = b,
        CfgMsg::UpperCase(b) => {
            if cfg.lower && b {
                cfg.lower = false
            }
            if !cfg.index_only {
                cfg.upper = b;
            } else {
                return true;
            }
        },
        CfgMsg::LowerCase(b) => {
            if cfg.upper && b {
                cfg.upper = false
            }
            if !cfg.index_only {
                cfg.lower = b;
            } else {
                return true;
            }
        },

        CfgMsg::IndexPadding(padding) => cfg.index_padding = padding,
        CfgMsg::DestinationFolder(destination) => cfg.destination = destination,
    }
    false
    // Command::none()
} 

#[derive(Default)]
pub struct XmoditsGui {
    cfg: core::cfg::Config,
    paths: Vec<String>,
    toggls: bool,
    audio: core::sfx::Audio,
    ripper: core::xmodits::Ripper,
    // ripper: TestOne
}

impl Application for XmoditsGui {
    type Message = Msg;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Msg>) {
        let c = ["it", "xm", "s3m", "mod", "umx"];
        let a = (1000..1200).into_iter().map(|d| format!("{}.{}",d.to_string(), c[d % c.len()])).collect();
        // println!("{:?}",&a);
        (
            Self{
                paths: a,
                cfg: Config::load(),
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("XMODITS")
    }

    fn update(&mut self, message: Msg) -> Command<Msg> {
        match message {
            Msg::Rip => todo!(),
            Msg::check(g) => self.toggls = g,
            Msg::SetCfg(cfg) => {
                if set_cfg(&mut self.cfg, cfg) {
                    self.audio.play("sfx_2")
                }
            },
            Msg::Beep(sfx) =>  self.audio.play(&sfx) ,
            Msg::StartRip => return Command::perform(
                async {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    String::from("sfx_1")
                },Msg::Beep
            ),
            Msg::OpenFileDialoge => return Command::perform(
                async {
                    // tokio::
                    match rfd::FileDialog::new()
                        .pick_file(){
                            Some(handle) => Some(handle),
                            None => None
                        }
                }, Msg::AddFile
            ),
            Msg::AddFile(path) => {
                if let Some(path) = path {
                    self.paths.push(format!("{}", path.display()));
                    self.audio.play("sfx_1");
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Msg, Renderer<Self::Theme>> {
        // let col = self.paths
        //     .iter()
        //     .enumerate()
        //     .fold(
        //         Column::new().spacing(10), |column, (index, path)| {
        //             column.push(checkbox(path, true, |b| { dbg!(b); Message::check(b)}))
        //         }
        //     );
        let total_modules: _ =  text(format!("Total Modules: {}", self.paths.len()));
        let trackers: _ = self
            .paths
            .iter()
            .fold(
                Column::new().spacing(10).padding(5),
                |s, gs| s.push(row![
                    button(text(&gs))
                        .style(style::button::Button::NormalPackage)
                        .on_press(Msg::Beep("sfx_1".into()))
                        .width(Length::Fill),
                    Space::with_width(Length::Units(15))
                ])
            );

        let buttonx = row![
            button("Add Module").padding(10).on_press(Msg::OpenFileDialoge),
            Space::with_width(Length::Fill),
            
            button("Start Ripping").padding(10).on_press(Msg::Beep("sfx_1".into())),
        ].spacing(10);

        let trackers = column![
            total_modules,
            scrollable(trackers).height(Length::Fill),
            buttonx
        ]
        .width(Length::FillPortion(1))
        .spacing(10)
        .align_items(Alignment::Center);

        use CfgMsg::*;

        let input: _ = text_input(
            "Destination", &self.cfg.destination, |s| Msg::SetCfg(DestinationFolder(s))
        ).padding(10).on_submit(Msg::Beep("sfx_1".into()));

        let set_destination: _ = row![
            input,
            // Space::with_width(Length::Units(5)),
            button("Open")
                .on_press(Msg::Beep("sfx_1".into()))
                .padding(10),
                // .style(style::button::Button::Refresh)
            button("Settings")
                .on_press(Msg::Beep("sfx_1".into()))
                .padding(10),
        ]
        .spacing(5)
        .width(Length::Fill);

        // let top_buttons = Row::new()
        //     .spacing(5)
        //     .padding(1)
        //     .push(Button::new("beep").on_press(Msg::Beep("sfx_1".into())))
        //     .push(Button::new("boop").on_press(Msg::Beep("sfx_2".into())));
        
        let settings = container(column![
            row![
                column![
                    checkbox("No Folder", self.cfg.no_folder, |b| Msg::SetCfg(NoFolder(b))),
                    checkbox("Index Only", self.cfg.index_only, |b| Msg::SetCfg(IndexOnly(b))),
                    checkbox("Preserve Index", self.cfg.index_raw, |b| Msg::SetCfg(IndexRaw(b))),
                ].spacing(2),
                column![
                    checkbox("Upper Case", self.cfg.upper, |b| Msg::SetCfg(UpperCase(b))),
                    checkbox("Lower Case", self.cfg.lower, |b| Msg::SetCfg(LowerCase(b))),
                    
                ].spacing(2)
            ].spacing(8),
            
            row![
                text("Padding"),  
                pick_list(vec![1,2,3], Some(self.cfg.index_padding), |b| Msg::SetCfg(IndexPadding(b))),
                    // .width(Length::Shrink)
            ].spacing(5),
            // .max_width(max_width)
            // .width(Length::FillPortion(1))
            
            // row![
            //     button("beep").on_press(Msg::Beep("sfx_1".into())),
            //     button("boop").on_press(Msg::Beep("sfx_2".into())),
            //     button("Open").on_press(Msg::OpenFileDialoge)
            // ].align_items(Alignment::Center)
        ]
        .spacing(5)
        )
            .style(style::Container::Frame)
            .padding(8)
            .width(Length::Fill);


        let top_panel: _ = row![
            // title,
            set_destination,
        ]
        .width(Length::FillPortion(1))
        .align_items(Alignment::Center);
        
        let stats: _ = scrollable(
            column![text("Module Name: NYC Streets"),
                text("Format: Impulse Tracker"),
                text("Samples: 26"),
                text("Approx Total Sample Size (KiB): 1532"),
                text("Comments: \n"),
                text(copypasta),
            ]
            .align_items(Alignment::Center)
            .spacing(5)
        )
        .height(Length::Fill)
        .style(style::scrollable::Scrollable::Dark);
        
        let main: _ = row![
            
            trackers,
            column![
                text("Configure Ripping:").font(JETBRAINS_MONO),
                // top_buttons,
                settings,
                text("Current Tracker Infomation:").font(JETBRAINS_MONO),
                container(
                    // column![
                        
                        stats,
                    // ]
                )
                .style(style::Container::Frame)
                .center_x()
                .padding(8)
                .width(Length::Fill)
                .height(Length::Fill)
                
            ]
            .width(Length::FillPortion(1))
            .spacing(10)

        ].spacing(10);

        let content = Column::new()
            .spacing(15)
            .height(Length::Fill)
            .push(top_panel)
            .push(main);
        
        // let bar = Column::new()
        //     .push(
        //         Row::new()
        //             .spacing(10)
        //             .push(Button::new("A").on_press(Msg::Beep("sfx_1".into())))
        //             .push(Button::new("B").on_press(Msg::Beep("sfx_2".into())))
        //             .push(Button::new("C").on_press(Msg::Beep("sfx_1".into())))
        //             .push(Button::new("D").on_press(Msg::Beep("sfx_2".into())))
        //     )
        //     .push(content);
            // .push(checkbox("des", self.sample_config.index_only, |b| Message::SetSampleConfig(IndexOnly(b))))
            
            // .push(col);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            // .center_x()
            // .center_y()
            .into()
    }
    // fn subscription(&self) -> Subscription<Msg> {
    //     time::Duration::from_secs(1).map(Msg::Beep)
    // }

}

impl XmoditsGui {
    pub fn start() {
        let settings: Settings<()> = Settings {
            window: Window {
                size: (650, 450),
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