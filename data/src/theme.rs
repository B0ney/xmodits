pub mod themes;

use iced::Color;
use serde::{Deserialize, Serialize};
pub use themes::Themes;

pub struct Theme {
    pub name: String,
    pub palette: Palette,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    pub background: Color,
    pub middleground: Color,
    pub foreground: Color,
    pub border: Color,
    pub text: Color,
    pub accent: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
    pub waveform: Color,
}

impl Default for Palette {
    fn default() -> Self {
        themes::Themes::Dark.palette()
    }
}
