//! Define what kind of files XMODITS should keep when scanning files

mod file_name;
mod file_size;
mod regex;

use data::config::filters::{Filter, Name, Size};
use std::path::Path;

use iced::widget::{column, horizontal_rule, row};

use crate::icon;
use crate::utils::{extension, filename};
use crate::widget::helpers::{control_filled, text_icon};
use crate::widget::{helpers::control, Element};

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

#[derive(Debug, Clone, Copy)]
pub enum Message {
    FileSize(file_size::Message),
    FileName(file_name::Message),
}

#[derive(Default, Debug)]
pub struct CustomFilters {
    filesize: Size,
    filename: Name,
}

impl CustomFilters {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::FileSize(filesize) => file_size::update(&mut self.filesize, filesize),
            Message::FileName(filename) => file_name::update(&mut self.filename, filename),
        }
    }
    pub fn view_file_size(&self) -> Element<Message> {
        file_size::view(&self.filesize).map(Message::FileSize)
    }

    pub fn view_file_name(&self) -> Element<Message> {
        file_name::view(&self.filename).map(Message::FileName)
    }
}

