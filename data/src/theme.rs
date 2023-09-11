pub mod themes;

use iced_core::Color;
// use serde::Deserialize;
pub use themes::Themes;

pub struct Theme {
    pub name: String,
    pub pallette: Palette,
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub base: BaseColors,
    pub normal: NormalColors,
    pub bright: BrightColors,
}

impl Default for Palette {
    fn default() -> Self {
        Themes::Catppuccin.palette()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BaseColors {
    pub background: Color,
    pub foreground: Color,
    pub dark: Color,   // TODO: sort
    pub border: Color, // TODO: sort
}

#[derive(Debug, Clone, Copy)]
pub struct NormalColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct BrightColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub error: Color,
}
