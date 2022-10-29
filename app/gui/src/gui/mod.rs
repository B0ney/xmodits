mod style;
// mod theme;

use std::path::PathBuf;
use crate::core;
use iced::Theme;
use iced::widget::{column, Container, Column, checkbox,Checkbox, pick_list, Row, Text, button, Button};
use iced::window::Icon;
use iced::{window::Settings as Window, Application, Command, Element, Length, Renderer, Settings};
use image::{self, GenericImageView};

#[derive(Debug, Clone)]
pub enum Msg {
    Rip,
    check(bool),
    SetCfg(CfgMsg),
    Beep(String)
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
    pub destination_folder: PathBuf,
}

impl SampleConfig{
    fn set(&mut self, msg: CfgMsg) {
        match msg {
            CfgMsg::NoFolder(b) => self.no_folder = b,
            CfgMsg::IndexOnly(b) => self.index_only = b,
            CfgMsg::IndexRaw(b) => self.index_raw = b,
            CfgMsg::UpperCase(b) => self.upper_case = b,
            CfgMsg::LowerCase(b) => self.lower_case = b,
            CfgMsg::IndexPadding(padding) => self.index_padding = padding,
            CfgMsg::DestinationFolder(destination) => self.destination_folder = PathBuf::from(destination),
        }
    } 
}

#[derive(Default)]
pub struct XmoditsGui {
    cfg: SampleConfig,
    paths: Vec<String>,
    toggls: bool,
    audio: core::sfx::Audio,
}

impl Application for XmoditsGui {
    type Message = Msg;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Msg>) {
        (
            Self{
                paths: vec!["dklsjaf;", "djlkajfd;la", "djslfjda"].into_iter().map(|d| d.to_string()).collect(),
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
            Msg::SetCfg(cfg) => self.cfg.set(cfg),
            Msg::Beep(sfx) => self.audio.play(&sfx),
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
        use CfgMsg::*;
        let content = Column::new()
            .spacing(10)
            // .max_width(max_width)
            .push(checkbox("No Folder", self.cfg.no_folder, |b| Msg::SetCfg(NoFolder(b))))
            .push(checkbox("Index Only", self.cfg.index_only, |b| Msg::SetCfg(IndexOnly(b))))
            .push(checkbox("Index Raw", self.cfg.index_raw, |b| Msg::SetCfg(IndexRaw(b))))
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
                Row::new().spacing(5)
                    .push(Button::new("beep").on_press(Msg::Beep("sfx_1".into())))
                    .push(Button::new("boop").on_press(Msg::Beep("sfx_2".into())))
            );

                
            // .push(checkbox("des", self.sample_config.index_only, |b| Message::SetSampleConfig(IndexOnly(b))))
            
            // .push(col);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }

}

impl XmoditsGui {
    pub fn start() {
        let settings: Settings<()> = Settings {
            window: Window {
                size: (400, 600),
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