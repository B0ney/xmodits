use iced::{Background, Border, Color};

/// The appearance of a waveform viewer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Appearance {
    pub background: Background,
    pub wave_color: Color,
    pub cursor_color: Color,
    pub border: Border,
}

pub trait StyleSheet {
    type Style: Default;

    fn appearance(&self, style: &Self::Style) -> Appearance;
}
