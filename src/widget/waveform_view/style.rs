use iced::{Background, BorderRadius, Color};

/// The appearance of a waveform viewer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Appearance {
    pub background: Background,
    pub wave_color: Color,
    pub cursor_color: Color,
    pub border_radius: BorderRadius,
    pub border_width: f32,
    pub border_color: Color,
}

pub trait StyleSheet {
    type Style: Default;

    fn appearance(&self, style: &Self::Style) -> Appearance;
}
