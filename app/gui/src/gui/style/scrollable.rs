use iced::widget::scrollable;
use iced::{Background, Color};
use super::Theme;

#[derive(Default, Debug, Clone, Copy)]
pub enum Scrollable {
    #[default]
    Description,
    Packages,
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, style: Self::Style) -> scrollable::Scrollbar {
        let from_appearance = |c: Color| scrollable::Scrollbar {
            background: Some(Background::Color(c)),
            border_radius: 5.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: self.palette().base.foreground,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        };
        from_appearance(self.palette().base.background)
        // match style {
        //     Scrollable::Description => ,
        //     Scrollable::Packages => from_appearance(self.palette().base.background),
        // }
    }

    fn hovered(&self, style: Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            scroller: scrollable::Scroller {
                ..self.active(style).scroller
            },
            ..self.active(style)
        }
    }

    fn dragging(&self, style: Self::Style) -> scrollable::Scrollbar {
        let hovered = self.hovered(style);
        scrollable::Scrollbar {
            scroller: scrollable::Scroller { ..hovered.scroller },
            ..hovered
        }
    }
}