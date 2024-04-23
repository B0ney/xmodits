/*
Copyright (c) 2024 B0ney

The `theme` module is dual licensed under MIT or Apache-2.0:
    * Apache 2.0 - https://www.apache.org/licenses/LICENSE-2.0
    * MIT - https://mit-license.org/
*/

pub mod themes;

use iced::Color;
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
