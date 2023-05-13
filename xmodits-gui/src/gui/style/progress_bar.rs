use iced::widget::progress_bar;
use iced::Background;

use super::Theme;

#[derive(Default, Debug, Clone, Copy)]
pub enum ProgressBar {
    #[default]
    Default,
}

impl progress_bar::StyleSheet for Theme {
    type Style = ProgressBar;

    fn appearance(&self, style: &Self::Style) -> progress_bar::Appearance {
        let p = self.palette();

        progress_bar::Appearance {
            background: Background::Color(p.base.background),
            bar: Background::Color(p.normal.primary),
            border_radius: 64.0,
        }
    }
}
