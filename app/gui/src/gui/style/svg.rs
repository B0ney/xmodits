use iced::widget::svg;
use iced::{color, Background, Color};

use super::Theme;

impl svg::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> svg::Appearance {
        let p = self.palette();

        svg::Appearance {
            color: Some(color!(0xffffff)),
        }
    }
}
