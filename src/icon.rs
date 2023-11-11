use iced::alignment;
use iced::widget::{image, text, Image};

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

pub fn stop<'a>() -> Text<'a> {
    icon('\u{f4f5}')
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

pub fn calendar<'a>() -> Text<'a> {
    icon('\u{e803}')
}

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(font::ICONS)
        .size(12.0)
        .horizontal_alignment(alignment::Horizontal::Center)
}

pub fn xmodits_logo() -> Image<image::Handle> {
    image(get_img("xmodits"))
}

pub fn vbee3() -> Image<image::Handle> {
    image(get_img("vbee3"))
}

fn get_img(src: &str) -> image::Handle {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    
    static MAP: Lazy<HashMap<&'static str, image::Handle>> = Lazy::new(|| {
        let load = |bytes: &'static [u8]| image::Handle::from_memory(bytes);

        HashMap::from([
            ("xmodits",load(include_bytes!("../assets/img/logos/icon.png"))),
            ("vbee3", load(include_bytes!("../assets/img/vbee3.png"))),
        ])
    });

    MAP.get(src)
        .expect(&format!("invalid key '{src}' provided"))
        .clone()
}
