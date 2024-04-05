pub mod application;
pub mod button;
pub mod checkbox;
pub mod container;
pub mod overlay;
pub mod pick_list;
pub mod progress_bar;
pub mod radio;
pub mod rule;
pub mod scrollable;
pub mod slider;
pub mod text_input;
pub mod text;
pub mod waveform_view;
pub mod helpers;

use data::theme;

const BORDER_RADIUS: f32 = 5.0;
const BORDER_WIDTH: f32 = 1.5;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Theme(pub theme::Palette);

impl Theme {
    pub fn palette(&self) -> &theme::Palette {
        &self.0
    }
}
