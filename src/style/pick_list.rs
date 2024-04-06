use iced::border::{Border, Radius};
use iced::widget::pick_list::{Catalog, Status, Style, StyleFn};
use iced::{color, Color};

use super::helpers::border;
use super::{Theme, BORDER_RADIUS, BORDER_WIDTH};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    match status {
        Status::Active => Style {
            text_color: p.text,
            placeholder_color: p.text,
            handle_color: p.text,
            background: p.middleground.into(),
            border: border(p.border),
        },
        Status::Hovered | Status::Opened => Style {
            text_color: p.text,
            placeholder_color: p.text,
            handle_color: p.text,
            background: p.middleground.into(),
            border: Border {
                color: p.accent,
                width: 2.0,
                radius: BORDER_RADIUS.into(),
            },
        },
    }
}
