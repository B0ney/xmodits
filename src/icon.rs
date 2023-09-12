use data::config::config_dir;
use iced::widget::text;
use crate::{font, theme};
// use crate::gui::style::Theme;

use crate::widget::Text;
use iced::{alignment, Renderer};
// use iced_gif::gif;
use once_cell::sync::Lazy;
use std::path::Path;
use tracing::{error, info};

// pub static GIF: Lazy<Animation> = Lazy::new(|| Animation::new());

// pub struct Animation {
//     pub idle: iced_gif::Frames,
//     pub ripping: iced_gif::Frames,
// }

// impl Animation {
//     pub fn init_lazy(&self) {}

//     /// Allow loading custom animations
//     pub fn new() -> Self {
//         let idle_gif = config_dir().join("idle.gif");
//         let ripping_gif = config_dir().join("ripping.gif");

//         let idle = Self::load(idle_gif).unwrap_or_else(|_| Self::default_idle());

//         let ripping = Self::load(ripping_gif).unwrap_or_else(|_| Self::default_ripping());

//         Self { idle, ripping }
//     }

//     fn load(path: impl AsRef<Path>) -> anyhow::Result<iced_gif::Frames> {
//         const MAX_SIZE: u64 = 2 * 1024 * 1024;

//         if std::fs::metadata(path.as_ref())?.len() > MAX_SIZE {
//             error!("Custom animation is over 2MB");
//             anyhow::bail!("")
//         }

//         let result = gif::Frames::from_bytes(std::fs::read(path.as_ref())?);

//         match result.as_ref().err() {
//             None => info!("Loaded custom animation!"),
//             Some(e) => error!("Failed to load custom animation: {}", e),
//         };

//         Ok(result?)
//     }

//     fn default_idle() -> iced_gif::Frames {
//         gif::Frames::from_bytes(include_bytes!("../assets/gif/white_lie_8fps.gif").to_vec())
//             .unwrap()
//     }

//     fn default_ripping() -> iced_gif::Frames {
//         gif::Frames::from_bytes(include_bytes!("../assets/gif/white_walk_8fps.gif").to_vec())
//             .unwrap()
//     }
// }

pub fn folder<'a>() -> Text<'a>{
    icon('\u{f3d1}')
}

pub fn download<'a>() -> Text<'a>  {
    icon('\u{f30a}')
}

pub fn play<'a>() -> Text<'a>  {
    icon('\u{f4f4}')
}

pub fn pause<'a>() -> Text<'a>  {
    icon('\u{f4c3}')
}

pub fn repeat<'a>() -> Text<'a>  {
    icon('\u{f813}')
}

pub fn github<'a>() -> Text<'a>  {
    icon('\u{f3ed}')
}

pub fn git<'a>() -> Text<'a>  {
    icon('\u{f69d}')
}

pub fn question<'a>() -> Text<'a>  {
    icon('\u{f504}')
}

pub fn warning<'a>() -> Text<'a>  {
    icon('\u{f33a}')
}

pub fn error<'a>() -> Text<'a>  {
    icon('\u{f622}')
}

pub fn filter<'a>() -> Text<'a>  {
    icon('\u{f3c4}')
}

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(font::ICONS)
        .size(20.0)
        .horizontal_alignment(alignment::Horizontal::Center)
}
