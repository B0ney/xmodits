use iced::border::{Border, Radius};
use iced::widget::rule::{Catalog, Style, StyleFn, FillMode};
use iced::{color, Color};

use super::{Theme, BORDER_RADIUS, BORDER_WIDTH};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn primary(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        color: p.border,
        width: 1,
        radius: 1.0.into(),
        fill_mode: FillMode::Full,
    }
}
