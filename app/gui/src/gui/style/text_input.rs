use iced::widget::text_input;
use iced::{Background, Color};
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
            border_radius: 5.0,
            border_width: 0.0,
            border_color: self.palette().base.foreground,
        }
    }

    fn focused(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(self.palette().base.foreground),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.palette().normal.primary
            },
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