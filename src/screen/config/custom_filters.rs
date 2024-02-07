//! Define what kind of files XMODITS should keep when scanning files

mod file_date;
mod file_name;
mod file_size;
mod regex;

use data::config::filters::{Filter, Name, Size};
use iced::Command;
use std::path::Path;

use iced::widget::{column, horizontal_rule, row};

use crate::icon;
use crate::utils::{extension, filename};
use crate::widget::helpers::{control_filled, text_icon};
use crate::widget::{helpers::control, Element};

use self::file_date::DateFilter;
use self::file_name::NameFilter;

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

#[derive(Debug, Clone)]
pub enum Message {
    FileSize(file_size::Message),
    FileName(file_name::Message),
    FileDate(file_date::Message),
}

#[derive(Default, Debug)]
pub struct CustomFilters {
    pub filesize: Size,
    pub filename: NameFilter,
    pub date: DateFilter,
}

impl CustomFilters {
    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::FileSize(filesize) => file_size::update(&mut self.filesize, filesize),
            Message::FileName(filename) => return self.filename.update(filename).map(Message::FileName),
            Message::FileDate(filedate) => self.date.update(filedate),
        }
        Command::none()
    }
    pub fn view_file_size(&self) -> Element<Message> {
        file_size::view(&self.filesize).map(Message::FileSize)
    }

    pub fn view_file_name(&self) -> Element<Message> {
        self.filename.view().map(Message::FileName)
    }

    pub fn view_file_date(&self) -> Element<Message> {
        self.date.view().map(Message::FileDate)
    }
}
