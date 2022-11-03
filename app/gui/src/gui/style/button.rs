use iced::widget::button;
use iced::{Background, Color};

use super::Theme;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    #[default]
    Primary,
    Unavailable,
    SelfUpdate,
    Refresh,
    UninstallPackage,
    RestorePackage,
    NormalPackage,
    SelectedPackage,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: Self::Style) -> button::Appearance {
        let p = self.palette();

        let appearance = button::Appearance {
            border_width: 1.0,
            border_radius: 2.0,
            ..button::Appearance::default()
        };

        let active_appearance = |bg: Option<Color>, mc| button::Appearance {
            background: Some(Background::Color(bg.unwrap_or(p.base.foreground))),
            border_color: Color { a: 0.5, ..mc },
            text_color: mc,
            ..appearance
        };

        match style {
            Button::Primary => active_appearance(None, p.bright.primary),
            Button::Unavailable => active_appearance(None, p.bright.error),
            Button::Refresh => active_appearance(None, p.bright.primary),
            Button::SelfUpdate => active_appearance(None, p.bright.primary),
            Button::UninstallPackage => active_appearance(None, p.bright.error),
            Button::RestorePackage => active_appearance(None, p.bright.secondary),
            Button::NormalPackage => button::Appearance {
                background: Some(Background::Color(p.base.foreground)),
                text_color: p.bright.surface,
                border_radius: 5.0,
                border_width: 0.0,
                border_color: p.base.background,
                ..appearance
            },
            Button::SelectedPackage => button::Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..p.normal.primary
                })),
                text_color: p.bright.primary,
                border_radius: 5.0,
                border_width: 0.0,
                border_color: p.normal.primary,
                ..appearance
            },
        }
    }

    fn hovered(&self, style: Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self.palette();

        let hover_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.25, ..bg })),
            text_color: tc.unwrap_or(bg),
            ..active
        };

        match style {
            Button::Primary => hover_appearance(p.bright.primary, None),
            Button::Unavailable => hover_appearance(p.bright.error, None),
            Button::Refresh => hover_appearance(p.bright.primary, None),
            Button::SelfUpdate => hover_appearance(p.bright.primary, None),
            Button::UninstallPackage => hover_appearance(p.bright.error, None),
            Button::RestorePackage => hover_appearance(p.bright.secondary, None),
            Button::NormalPackage => hover_appearance(p.normal.primary, Some(p.bright.surface)),
            Button::SelectedPackage => hover_appearance(p.normal.primary, None),
        }
    }

    fn disabled(&self, style: Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self.palette();

        let disabled_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.05, ..bg })),
            text_color: Color {
                a: 0.50,
                ..tc.unwrap_or(bg)
            },
            ..active
        };

        match style {
            Button::RestorePackage => disabled_appearance(p.normal.primary, Some(p.bright.primary)),
            Button::UninstallPackage => disabled_appearance(p.bright.error, None),
            _ => button::Appearance { ..active },
        }
    }

    fn pressed(&self, style: Self::Style) -> button::Appearance {
        button::Appearance {
            ..self.active(style)
        }
    }
}