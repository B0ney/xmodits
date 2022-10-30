mod style;
// mod theme;
use std::path::PathBuf;
use std::time::Duration;
use crate::core;
use crate::core::font::JETBRAINS_MONO;
use iced::{Theme, Alignment, Subscription, time};
use iced::widget::{column, Container, Column, checkbox,Checkbox, pick_list, Row, Text, button, Button, row, scrollable, text_input, text};
use iced::window::Icon;
use iced::{window::Settings as Window, Application, Command, Element, Length, Renderer, Settings};
use image::{self, GenericImageView};

use rfd::AsyncFileDialog;

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

#[derive(Default, Clone)]
pub struct SampleConfig {
    pub no_folder: bool,
    pub index_only: bool,
    pub index_raw: bool,
    pub upper_case: bool,
    pub lower_case: bool,
    pub index_padding: usize,
    pub destination_folder: String,
}

impl SampleConfig{
    fn set(&mut self, msg: CfgMsg) -> bool {
        match msg {
            CfgMsg::NoFolder(b) => self.no_folder = b,
            CfgMsg::IndexOnly(b) => {
                if b {
                    self.upper_case = false;
                    self.lower_case = false;
                }
                self.index_only = b;
            },
            CfgMsg::IndexRaw(b) => self.index_raw = b,
            CfgMsg::UpperCase(b) => {
                if self.lower_case && b {
                    self.lower_case = false
                }
                if !self.index_only {
                    self.upper_case = b;
                } else {
                    return true;
                }
            },
            CfgMsg::LowerCase(b) => {
                if self.upper_case && b {
                    self.upper_case = false
                }
                if !self.index_only {
                    self.lower_case = b;
                } else {
                    return true;
                }
            },

            CfgMsg::IndexPadding(padding) => self.index_padding = padding,
            CfgMsg::DestinationFolder(destination) => self.destination_folder = destination,
        }
        false
        // Command::none()
    } 
}

#[derive(Default)]
pub struct XmoditsGui {
    cfg: SampleConfig,
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
        let a = (1..12).into_iter().map(|d| format!("{}.{}",d.to_string(), c[d % c.len()])).collect();
        // println!("{:?}",&a);
        (
            Self{
                paths: a,
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
                if self.cfg.set(cfg) {
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
                    match AsyncFileDialog::new()
                        .pick_file()
                        .await {
                            Some(handle) => Some(handle.path().to_owned()),
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

    fn view(&self) -> Element<Self::Message> {
        // let col = self.paths
        //     .iter()
        //     .enumerate()
        //     .fold(
        //         Column::new().spacing(10), |column, (index, path)| {
        //             column.push(checkbox(path, true, |b| { dbg!(b); Message::check(b)}))
        //         }
        //     );
        let title: _ =  Text::new(format!("Modules: {}", self.paths.len()));
        let trackers: _ = self
            .paths
            .iter()
            .fold(
                Column::new().spacing(10),
                |s,gs| { s.push(Text::new(gs).font(JETBRAINS_MONO)) }
            ).width(Length::FillPortion(1));

        let scrollable = Column::new()
            .width(Length::FillPortion(1))
            .spacing(5)
            .push(title)    
            .push(scrollable(trackers));

        use CfgMsg::*;
        let input: _ = text_input(
            "Destination", &self.cfg.destination_folder, |s| Msg::SetCfg(DestinationFolder(s))
        ).padding(10).on_submit(Msg::Beep("sfx_1".into()));

        
        let settings = Column::new()
            .spacing(5)
            // .max_width(max_width)
            .width(Length::FillPortion(1))
            .push(checkbox("No Folder", self.cfg.no_folder, |b| Msg::SetCfg(NoFolder(b))))
            .push(checkbox("Index Only", self.cfg.index_only, |b| Msg::SetCfg(IndexOnly(b))))
            .push(checkbox("Preserve Index", self.cfg.index_raw, |b| Msg::SetCfg(IndexRaw(b))))
            .push(checkbox("Upper Case", self.cfg.upper_case, |b| Msg::SetCfg(UpperCase(b))))
            .push(checkbox("Lower Case", self.cfg.lower_case, |b| Msg::SetCfg(LowerCase(b))))
            .push(
                Row::new()
                    .spacing(5)
                    .push(Text::new("Index Padding"))
                    .push(
                        pick_list(vec![1,2,3], Some(self.cfg.index_padding), |b| Msg::SetCfg(IndexPadding(b)))
                    )
                )
            .push(
                Column::new()
                .align_items(Alignment::Center)
                .push(
                    Row::new()
                        // .align_items(Alignment::End)
                        .spacing(10)
                        .push(Button::new("beep").on_press(Msg::Beep("sfx_1".into())))
                        .push(Button::new("boop").on_press(Msg::Beep("sfx_2".into())))
                        .push(Button::new("Open").on_press(Msg::OpenFileDialoge))
                        // .push(Button::new(text("boned").font(JETBRAINS_MONO)).on_press(Msg::Beep("sfx_3".into())))
                        // .push(Button::new("aauugghh").on_press(Msg::Beep("sfx_4".into())))
                )
            )
            .push(input);

        let content = Row::new()
            .spacing(20)
            .height(Length::Fill)
            .push(scrollable)
            .push(settings);
        
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
            .padding(20)
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

fn icon() -> Icon {
    let image = image::load_from_memory(include_bytes!("../../../../extras/logos/png/icon3.png")).unwrap();
    let (w, h) = image.dimensions();
    Icon::from_rgba(image.as_bytes().to_vec(), w, h).unwrap()
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