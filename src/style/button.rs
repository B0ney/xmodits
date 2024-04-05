use iced::border::{Border, Radius};
use iced::widget::button::{Catalog, Status, Style, StyleFn};
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
    todo!()
}

pub fn cancel(theme: &Theme, status: Status) -> Style {
    todo!()
}

pub fn hyperlink(theme: &Theme, status: Status) -> Style {
    todo!()
}

pub fn hyperlink_inverted(theme: &Theme, status: Status) -> Style {
    todo!()
}

pub fn entry(theme: &Theme, status: Status) -> Style {
    todo!()
}

pub fn entry_error(theme: &Theme, status: Status) -> Style {
    todo!()
}

pub fn start(theme: &Theme, status: Status) -> Style {
    todo!()
}

pub fn delete(theme: &Theme, status: Status) -> Style {
    todo!()
}