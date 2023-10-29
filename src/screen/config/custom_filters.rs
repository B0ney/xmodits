//! Define what kind of files XMODITS should keep when scanning files

mod file_name;
mod file_size;

use std::path::Path;
use data::config::filters::{Filter, Name, Size};

use iced::widget::{column, horizontal_rule, row};

use crate::icon;
use crate::utils::{extension, filename};
use crate::widget::{helpers::control, Element};

// fn
pub struct Filters(Vec<Box<dyn Filter>>);

impl Filters {
    pub fn new(filters: Vec<Box<dyn Filter>>) -> Self {
        Self(filters)
    }

    pub fn matches(&self, path: &Path) -> bool {
        for filter in &self.0 {
            if !filter.matches(path) {
                return false;
            }
        }
        true
    }
}

pub enum Message {}

pub fn view<'a>() -> Element<'a, Message> {
    let title = row!["Filters", icon::filter()].spacing(8);

    let menu = |title: &'a str| {
        row![title, horizontal_rule(1)]
            .spacing(8)
            .align_items(iced::Alignment::Center)
    };

    let settings = column![
        menu("Size"),
        // horizontal_rule(1),
        menu("Extension"),
        // horizontal_rule(1),
        menu("Name"),
        // horizontal_rule(1),
        menu("Date"),
    ]
    .spacing(8);

    control(title, settings).into()
}
