use iced::color;
use iced::daemon::{Appearance, DefaultStyle};

use super::Theme;

impl DefaultStyle for Theme {
    fn default_style(&self) -> Appearance {
        Appearance {
            background_color: self.palette().middleground,
            text_color: self.palette().text,
        }
    }
}