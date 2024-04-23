/*
Copyright (c) 2024 B0ney

The `waveform_view` module is dual licensed under MIT or Apache-2.0:
    * Apache 2.0 - https://www.apache.org/licenses/LICENSE-2.0
    * MIT - https://mit-license.org/
*/

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
