use iced::widget::text_input;
use iced::{Background, Color, color};
use super::Theme;


#[derive(Default, Debug, Clone, Copy)]
pub enum TextInput {
    #[default]
    Default,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.palette().base.foreground),
            border_radius: 8.0,
            border_width: 1.2,
            border_color: color!(0x474747),
        }
    }

    fn focused(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.palette().base.foreground),
            border_radius: 8.0,
            border_width: 1.2,
            border_color: self.palette().normal.primary,
        }
    }

    fn placeholder_color(&self, _style: Self::Style) -> Color {
        self.palette().normal.surface
    }

    fn value_color(&self, _style: Self::Style) -> Color {
        self.palette().bright.primary
    }

    fn selection_color(&self, _style: Self::Style) -> Color {
        self.palette().normal.primary
    }

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}