use iced::widget::{slider::{self, Appearance, Handle, HandleShape}, slider::Rail};
use super::Theme;

#[derive(Default)]
pub enum Style {
    #[default]
    Default
}

impl slider::StyleSheet for Theme {
    type Style = Style;

    fn active(&self, style: &Self::Style) -> Appearance {
        let p = self.palette();
        Appearance {
            rail: Rail {
                colors: (p.normal.primary, p.normal.primary),
                width: 3.0,
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 3.0 },
                color: p.normal.primary,
                border_width: 3.0,
                border_color: p.normal.primary,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        let p = self.palette();
        Appearance {
            rail: Rail {
                colors: (p.normal.primary, p.normal.primary),
                width: 3.0,
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 3.0 },
                color: p.normal.primary,
                border_width: 3.0,
                border_color: p.normal.primary,
            },
        }
    }

    fn dragging(&self, style: &Self::Style) -> Appearance {
        let p = self.palette();
        Appearance {
            rail: Rail {
                colors: (p.normal.primary, p.normal.primary),
                width: 3.0,
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 3.0 },
                color: p.normal.primary,
                border_width: 3.0,
                border_color: p.normal.primary,
            },
        }
    }
}