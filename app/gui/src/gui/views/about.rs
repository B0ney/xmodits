use iced::{Element, Renderer, widget::container, Length};
use iced::widget::{text, pick_list,checkbox,column, row};

use crate::{gui::style::{self, Theme}, core::cfg::Config};
use crate::gui::JETBRAINS_MONO;

#[derive(Default)]
pub struct AboutView;

#[derive(Debug, Clone)]
pub enum Message{}

impl AboutView {
    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let about: _ = container(row![
            text("Xmodits - by B0ney")
        ])
        .style(style::Container::Frame)
        .padding(8)
        .width(Length::Fill)
        .height(Length::Fill);
        
        container(
            column![
                text("About").font(JETBRAINS_MONO),
                about
            ]
            .spacing(5)
        )
        .width(Length::Fill)
        .into()
        // todo!()
    }
}