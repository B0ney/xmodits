use super::ColorPalette;
use iced::widget::scrollable;
use iced::{color, Background, Color};

#[derive(Default, Debug, Clone, Copy)]
pub enum Scrollable {
    #[default]
    Description,
    Dark,
}

impl scrollable::StyleSheet for ColorPalette {
    type Style = Scrollable;

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        let from_appearance = |c: Color, d: Color| scrollable::Scrollbar {
            background: Some(Background::Color(c)),
            border_radius: 5.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: d,
                border_radius: 5.0,
                border_width: 1.0,
                border_color: self.base.border,
            },
        };
        //
        let color = (
            self.base.background,
            self.base.foreground,
        );
        match style {
            Scrollable::Description => from_appearance(color.0, color.1),
            Scrollable::Dark => from_appearance(color.1, color.0),
        }
    }

    fn hovered(&self, style: &Self::Style, hovered: bool) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            scroller: scrollable::Scroller {
                color: if hovered {
                    self.normal.primary
                } else {
                    self.active(style).scroller.color
                },
                ..self.active(style).scroller
            },
            ..self.active(style)
        }
    }

    fn dragging(&self, style: &Self::Style) -> scrollable::Scrollbar {
        let hovered = self.hovered(style, true);
        scrollable::Scrollbar {
            scroller: scrollable::Scroller { ..hovered.scroller },
            ..hovered
        }
    }
}
