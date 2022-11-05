
use iced::{Element, Renderer, widget::container, Length};
use iced::widget::{text, pick_list,checkbox,column, row};
use iced::widget::Space;
use crate::{gui::style::{self, Theme}, core::cfg::Config};
use crate::gui::JETBRAINS_MONO;

#[derive(Debug, Clone)]
pub enum Message {
    NoFolder(bool),
    IndexOnly(bool),
    IndexRaw(bool),
    UpperCase(bool),
    LowerCase(bool),
    IndexPadding(usize),
    DestinationFolder(String),
}

#[derive(Debug, Clone)]
pub struct ConfigView {
    pub cfg: Config
}
impl Default for ConfigView {
    fn default() -> Self {
        Self { cfg: Config::load() }
    }
}

impl ConfigView {
    pub fn update(&mut self, msg: Message) -> bool {
        match msg {
            Message::NoFolder(b) => self.cfg.no_folder = b,
            Message::IndexOnly(b) => {
                if b {
                    self.cfg.upper = false;
                    self.cfg.lower = false;
                }
                self.cfg.index_only = b;
            },
            Message::IndexRaw(b) => self.cfg.index_raw = b,
            Message::UpperCase(b) => {
                if self.cfg.lower && b {
                    self.cfg.lower = false
                }
                if !self.cfg.index_only {
                    self.cfg.upper = b;
                } else {
                    return true;
                }
            },
            Message::LowerCase(b) => {
                if self.cfg.upper && b {
                    self.cfg.upper = false
                }
                if !self.cfg.index_only {
                    self.cfg.lower = b;
                } else {
                    return true;
                }
            },
    
            Message::IndexPadding(padding) => self.cfg.index_padding = padding,
            Message::DestinationFolder(destination) => self.cfg.destination = destination,
        }
        false
    }

    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let settings: _ = container(
            // column![
            row![
                column![
                    checkbox("No Folder", self.cfg.no_folder, |b| Message::NoFolder(b)),
                    checkbox("Index Only", self.cfg.index_only, |b| Message::IndexOnly(b)),
                    checkbox("Preserve Index", self.cfg.index_raw, |b| Message::IndexRaw(b)),
                ]
                .spacing(8),
                column![
                    checkbox("Upper Case", self.cfg.upper, |b| Message::UpperCase(b)),
                    checkbox("Lower Case", self.cfg.lower, |b| Message::LowerCase(b)),
                    // row![
                    //     pick_list(vec![1,2,3], Some(self.cfg.index_padding), |b| Message::IndexPadding(b)),
                    //     text("Padding"),
                    // ].spacing(5).width(Length::Shrink),
                ]
                .spacing(8)
            ].spacing(8),
            
        // ].spacing(8)
        )
        .style(style::Container::Frame)
        .padding(8)
        .width(Length::Fill);

        container(
            column![
                text("Ripping Configuration").font(JETBRAINS_MONO),
                settings
            ]
            .spacing(5)
        )
        .width(Length::Fill)
        .into()
    }
}