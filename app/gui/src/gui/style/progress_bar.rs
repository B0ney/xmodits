use iced::widget::progress_bar;
use iced::{color, Background, Color};

use super::Theme;

#[derive(Default, Debug, Clone, Copy)]
pub enum ProgressBar {
    #[default]
    Default
}

impl progress_bar::StyleSheet for Theme {
    type Style = ProgressBar;

    fn appearance(&self, style: &Self::Style) -> progress_bar::Appearance {
        let p = self.palette();

        progress_bar::Appearance {
            background: Background::Color(p.base.foreground),
            bar: Background::Color(p.base.background),
            border_radius: 10.0,
        }
    }
}