use iced::border::{Border, Radius};
use iced::widget::text::{Catalog, Style, StyleFn};
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

pub fn primary(_: &Theme) -> Style {
    Style::default()
}

pub fn warning(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().warning),
    }
}

pub fn error(theme: &Theme) -> Style {
    Style {
        color: Some(theme.palette().error),
    }
}