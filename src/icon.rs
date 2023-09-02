use crate::core::cfg::config_dir;
use crate::{font, theme};
use crate::gui::style::Theme;

use iced::widget::{text, Text};
use iced::{alignment, Renderer};
use iced_gif::gif;
use once_cell::sync::Lazy;
use std::path::Path;
use tracing::{error, info};

pub static GIF: Lazy<Animation> = Lazy::new(|| Animation::new());

pub struct Animation {
    pub idle: iced_gif::Frames,
    pub ripping: iced_gif::Frames,
}

impl Animation {
    pub fn init_lazy(&self) {}

    /// Allow loading custom animations
    pub fn new() -> Self {
        let idle_gif = config_dir().join("idle.gif");
        let ripping_gif = config_dir().join("ripping.gif");

        let idle = Self::load(idle_gif).unwrap_or_else(|_| Self::default_idle());

        let ripping = Self::load(ripping_gif).unwrap_or_else(|_| Self::default_ripping());

        Self { idle, ripping }
    }

    fn load(path: impl AsRef<Path>) -> anyhow::Result<iced_gif::Frames> {
        const MAX_SIZE: u64 = 2 * 1024 * 1024;

        if std::fs::metadata(path.as_ref())?.len() > MAX_SIZE {
            error!("Custom animation is over 2MB");
            anyhow::bail!("")
        }

        let result = gif::Frames::from_bytes(std::fs::read(path.as_ref())?);

        match result.as_ref().err() {
            None => info!("Loaded custom animation!"),
            Some(e) => error!("Failed to load custom animation: {}", e),
        };

        Ok(result?)
    }

    fn default_idle() -> iced_gif::Frames {
        gif::Frames::from_bytes(include_bytes!("../assets/gif/white_lie_8fps.gif").to_vec())
            .unwrap()
    }

    fn default_ripping() -> iced_gif::Frames {
        gif::Frames::from_bytes(include_bytes!("../assets/gif/white_walk_8fps.gif").to_vec())
            .unwrap()
    }
}

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
// pub fn delete_icon() -> Text<'static, Renderer<Theme>> {
//     icon('\u{F1F8}')
// }

// pub fn github_icon() -> Text<'static, Renderer<Theme>> {
//     icon('\u{f345}')
// }

pub fn folder_icon<'a>() -> Text<'a>{
    icon('\u{f228}')
}

// pub fn folder_line_icon() -> Text<'static, Renderer<Theme>> {
//     icon('\u{f224}')
// }

// pub fn add_file_icon() -> Text<'static, Renderer<Theme>> {
//     icon('\u{f221}')
// }

pub fn download_icon<'a>() -> Text<'a>  {
    icon('\u{f220}')
}

// pub fn settings_icon() -> Text<'static, Renderer<Theme>> {
//     icon('\u{f3b8}')
// }

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(font::ICONS)
        .width(20)
        .horizontal_alignment(alignment::Horizontal::Center)
}
