use iced::alignment;
use iced::widget::{image, text};

use crate::font;
use crate::widget::Text;

pub fn folder<'a>() -> Text<'a> {
    icon('\u{f3d1}')
}

pub fn download<'a>() -> Text<'a> {
    icon('\u{f30a}')
}

pub fn play<'a>() -> Text<'a> {
    icon('\u{f4f4}')
}

pub fn pause<'a>() -> Text<'a> {
    icon('\u{f4c3}')
}

pub fn repeat<'a>() -> Text<'a> {
    icon('\u{f813}')
}

pub fn github<'a>() -> Text<'a> {
    icon('\u{f3ed}')
}

pub fn git<'a>() -> Text<'a> {
    icon('\u{f69d}')
}

pub fn question<'a>() -> Text<'a> {
    icon('\u{f504}')
}

pub fn warning<'a>() -> Text<'a> {
    icon('\u{f33a}')
}

pub fn error<'a>() -> Text<'a> {
    icon('\u{f622}')
}

pub fn filter<'a>() -> Text<'a> {
    icon('\u{f3c4}')
}

pub fn save<'a>() -> Text<'a> {
    icon('\u{e802}')
}

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(font::ICONS)
        .size(12.0)
        .horizontal_alignment(alignment::Horizontal::Center)
}

pub fn xmodits_logo() -> image::Handle {
    use std::sync::OnceLock;
    static HANDLE: OnceLock<image::Handle> = OnceLock::new();

    HANDLE
        .get_or_init(|| image::Handle::from_memory(include_bytes!("../assets/img/logos/icon.png")))
        .clone()
}
