use crate::core::font::ICONS;
use crate::gui::style::Theme;
use iced::widget::{svg, text, Text};
use iced::{alignment, Length, Renderer};

// static FOLDER: &[u8] = include_bytes!("../../res/img/svg/folder.svg");
// static FOLDER_ADD: &[u8] = include_bytes!("../../res/img/svg/folder_add.svg");
// static FILE_ADD: &[u8] = include_bytes!("../../res/img/svg/file_add.svg");
// static CLEAR: &[u8] = include_bytes!("../../res/img/svg/clear.svg");
// static GPLV3: &[u8] = include_bytes!("../../res/img/svg/gpl-v3-logo.svg");
// static GITHUB: &[u8] = include_bytes!("../../res/img/svg/github-mark-white.svg");

// type SVG = iced::widget::Svg<Renderer<Theme>>;

// pub fn svg_icon(bytes: &'static [u8]) -> SVG {
//     svg(svg::Handle::from_memory(bytes))
// }
// // pub fn

// pub fn add_folder_icon() -> SVG {
//     svg_icon(FOLDER_ADD)
//         .width(Length::Units(25))
//         .height(Length::Units(25))
// }

// pub fn add_file_icon() -> SVG {
//     svg_icon(FILE_ADD)
//         .width(Length::Units(25))
//         .height(Length::Units(25))
// }

// pub fn clear_icon() -> SVG {
//     svg_icon(CLEAR)
// }

// pub fn gpl3_icon() -> SVG {
//     svg_icon(GPLV3)
// }

// pub fn folder_icon() -> SVG {
//     svg_icon(FOLDER)
// }

// pub fn github_icon() -> SVG {
//     svg_icon(GITHUB)
//         .width(Length::Units(25))
//         .height(Length::Units(25))
// }
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

// pub fn settings_icon() -> Text<'static, Renderer<Theme>> {
//     icon('\u{f3b8}')
// }

fn icon(unicode: char) -> Text<'static, Renderer<Theme>> {
    text(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(alignment::Horizontal::Center)
}
