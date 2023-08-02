use iced::widget::button;
use iced::{Background, Color};

use super::ColorPalette;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    #[default]
    Primary,
    Cancel,
    Hyperlink,
    HyperlinkInverted,
    Unavailable,
    Entry,
    Start,
    Delete,
    // SelectedPackage,
}

impl button::StyleSheet for ColorPalette {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let p = self;

        let appearance = button::Appearance {
            border_width: 1.0,
            border_radius: 8.0.into(),
            ..button::Appearance::default()
        };

        let active_appearance = |bg: Option<Color>, mc| button::Appearance {
            background: Some(Background::Color(bg.unwrap_or(p.base.foreground))),
            border_color: Color { a: 0.5, ..mc },
            // border_radius: 0.0,
            text_color: mc,
            ..appearance
        };

        match style {
            Button::Primary => active_appearance(None, p.bright.primary),
            Button::Cancel => active_appearance(None, p.bright.error),
            Button::Unavailable => active_appearance(None, p.bright.error),
            Button::Entry => button::Appearance {
                background: Some(Background::Color(p.base.foreground)),
                text_color: p.bright.surface,
                border_radius: 5.0.into(),
                border_width: 1.0,
                border_color: self.base.border,
                ..appearance
            },
            // Button::SelectedPackage => button::Appearance {
            //     background: Some(Background::Color(Color {
            //         a: 0.25,
            //         ..p.normal.primary
            //     })),
            //     text_color: p.bright.primary,
            //     border_radius: 5.0,
            //     border_width: 1.0,
            //     border_color: color!(0x474747),

            //     // border_color: p.normal.primary,
            //     ..appearance
            // },
            Button::Hyperlink => button::Appearance {
                background: None,
                text_color: p.bright.surface,
                ..appearance
            },
            Button::Start => active_appearance(None, p.bright.secondary),
            Button::Delete => active_appearance(None, p.bright.error),

            Button::HyperlinkInverted => button::Appearance {
                background: None,
                text_color: p.bright.primary,

                // text_color: ,
                ..appearance
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self;

        let hover_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.25, ..bg })),

            text_color: tc.unwrap_or(bg),
            ..active
        };

        match style {
            Button::Primary => hover_appearance(p.bright.primary, Some(p.bright.surface)),
            Button::Unavailable => hover_appearance(p.bright.error, None),
            Button::Entry => hover_appearance(p.bright.primary, Some(p.bright.surface)),
            Button::Hyperlink => button::Appearance {
                background: None,
                ..hover_appearance(p.bright.primary, None)
            },
            Button::Start => hover_appearance(p.bright.secondary, Some(p.bright.surface)),
            Button::Delete => hover_appearance(p.bright.error, Some(p.bright.surface)),
            Button::HyperlinkInverted => button::Appearance {
                background: None,
                text_color: p.bright.surface,
                ..hover_appearance(p.bright.primary, None)
            },
            Button::Cancel => hover_appearance(p.bright.error, Some(p.bright.surface)),
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self;

        let disabled_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.05, ..bg })),
            text_color: Color {
                a: 0.50,
                ..tc.unwrap_or(bg)
            },
            ..active
        };

        match style {
            // Button::RestorePackage => disabled_appearance(p.normal.primary, Some(p.bright.primary)),
            // Button::UninstallPackage => disabled_appearance(p.bright.error, None),
            _ => button::Appearance { ..active },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self.active(style)
        }
    }
}
