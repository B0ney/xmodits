use iced::border::{Border, Radius};
use iced::widget::container::{Catalog, Style, StyleFn};
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
    todo!()
}

pub fn hovered<'a>(hovered: bool) ->  StyleFn<'a, Theme> {
    todo!()
}

pub fn frame(theme: &Theme) -> Style {
    todo!()
}

pub fn black(theme: &Theme) -> Style {
    todo!()
}

pub fn black_hovered<'a>(hovered: bool) ->  StyleFn<'a, Theme> {
    todo!()
}

