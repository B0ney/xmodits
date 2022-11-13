use iced::{Element, Renderer, widget::container, Length};
use iced::widget::{text, pick_list,checkbox,column, row, button};
use iced::widget::svg;
use crate::gui::icons::{github_icon};
use crate::{gui::style::{self, Theme}, core::cfg::Config};
use crate::gui::JETBRAINS_MONO;

#[derive(Default)]
pub struct AboutView;

#[derive(Debug, Clone)]
pub enum Message{
    GH
}

impl AboutView {
    pub fn update(&self, msg: Message) {
        match msg {
            Message::GH => {
                if open::that("https://github.com/B0ney/xmodits").is_err() {
                    
                };
            },
        }
    }
    pub fn view(&self) -> Element<Message, Renderer<Theme>> {
        let logo:_ = text("0.0.7-Alpha").font(JETBRAINS_MONO);
        let gh: _ = button(github_icon().size(20)).on_press(Message::GH);
        let about: _ = container(column![
            text("Xmodits - by B0ney"),
            logo,
            gh,
            // svg::Svg::load(include_bytes!("../../res/img/agpl3_logo.svg"))
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
            .spacing(10)
        )
        .width(Length::Fill)
        .into()
    }
}