use iced::border::{Border, Radius};
use iced::widget::progress_bar::{Catalog, Style, StyleFn};
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
    Style {
        background: theme.palette().middleground.into(),
        bar: theme.palette().accent.into(),
        border: Border {
            color: theme.palette().border,
            width: 15.0,
            radius: 64.0.into(),
        },
    }
}
