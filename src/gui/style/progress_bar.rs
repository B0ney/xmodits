use iced::widget::progress_bar;
use iced::Background;

use super::ColorPalette;

#[derive(Default, Debug, Clone, Copy)]
pub enum ProgressBar {
    #[default]
    Default,
}

impl progress_bar::StyleSheet for ColorPalette {
    type Style = ProgressBar;

    fn appearance(&self, style: &Self::Style) -> progress_bar::Appearance {
        progress_bar::Appearance {
            background: Background::Color(self.base.background),
            bar: Background::Color(self.normal.primary),
            border_radius: 64.0.into(),
        }
    }
}
