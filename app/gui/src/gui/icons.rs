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

fn icon(unicode: char) -> Text<'static, Renderer<Theme>> {
    text(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(alignment::Horizontal::Center)
    // .size(20)
}
