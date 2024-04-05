use iced::border::{Border, Radius};
use iced::widget::checkbox::{Catalog, Status, Style, StyleFn};
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

pub fn primary(theme: &Theme, status: Status) -> Style {
    let p = theme.inner();

    Style {
        background: p.middleground.into(),
        icon_color: p.accent,
        border: Border {
            color: p.border,
            width: BORDER_WIDTH,
            radius: BORDER_RADIUS.into(),
        },
        text_color: Some(p.text),
    }
}

pub fn inverted(theme: &Theme, status: Status) -> Style {
    todo!()
}

pub fn entry(theme: &Theme, status: Status) -> Style {
    todo!()
}