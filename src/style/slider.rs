use iced::border::{Border, Radius};
use iced::widget::slider::{Catalog, Handle, HandleShape, Rail, Status, Style, StyleFn};
use iced::{color, Color};

use super::{Theme, BORDER_RADIUS, BORDER_WIDTH};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &Theme, _: Status) -> Style {
    let p = theme.palette();

    Style {
        rail: Rail {
            colors: (p.accent, p.accent),
            width: 3.0,
            border_radius: Default::default(),
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: 3.0 },
            color: p.accent,
            border_width: 3.0,
            border_color: p.accent,
        },
    }
}
