use iced::border::{Border, Radius};
use iced::widget::overlay::menu::{Catalog, Style, StyleFn};
use iced::{color, Color};

use super::helpers::border;
use super::{Theme, BORDER_RADIUS, BORDER_WIDTH};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
        class(self)
    }
}

pub fn primary(theme: &Theme) -> Style {
    let p = theme.palette();
    Style {
        background: p.middleground.into(),
        border: border(p.border),
        text_color: p.text,
        selected_text_color: p.text,
        selected_background: Color { a: 0.5, ..p.accent }.into(),
    }
}
