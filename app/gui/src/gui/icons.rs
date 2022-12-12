use crate::core::font::ICONS;
use iced::widget::{text, Text};
use iced::{alignment, Length, Renderer};

use super::style::Theme;

pub fn delete_icon() -> Text<'static, Renderer<Theme>> {
    icon('\u{F1F8}')
}

pub fn github_icon() -> Text<'static, Renderer<Theme>> {
    icon('\u{f345}')
}

pub fn folder_icon() -> Text<'static, Renderer<Theme>> {
    icon('\u{f228}')
}

pub fn folder_line_icon() -> Text<'static, Renderer<Theme>> {
    icon('\u{f224}')
}

pub fn add_file_icon() -> Text<'static, Renderer<Theme>> {
    icon('\u{f221}')
}

pub fn download_icon() -> Text<'static, Renderer<Theme>> {
    icon('\u{f220}')
}

pub fn settings_icon() -> Text<'static, Renderer<Theme>> {
    icon('\u{f3b8}')
}

fn icon(unicode: char) -> Text<'static, Renderer<Theme>> {
    text(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(alignment::Horizontal::Center)
    // .size(20)
}
