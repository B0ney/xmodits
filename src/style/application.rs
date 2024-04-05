use iced::color;
use iced::program::{Appearance, DefaultStyle};

use super::Theme;

impl DefaultStyle for Theme {
    fn default_style(&self) -> Appearance {
        Appearance {
            background_color: self.inner().middleground,
            text_color: self.inner().text,
        }
    }
}