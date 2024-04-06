use iced::border::{Border, Radius};
use iced::widget::button::{Catalog, Status, Style, StyleFn};
use iced::{color, Color};

use super::helpers::border;
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
    let p = theme.palette();

    match status {
        Status::Active | Status::Pressed | Status::Disabled=> Style {
            background: Some(p.foreground.into()),
            text_color: p.accent,
            border: border(Color { a: 0.5, ..p.accent }),
            ..Default::default()
        },
        Status::Hovered => Style {
            background: Some(Color { a: 0.4, ..p.accent }.into()),
            text_color: p.text,
            border: border(Color { a: 0.5, ..p.accent }),
            ..Default::default()
        },
    }
}

pub fn cancel(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    match status {
        Status::Active | Status::Pressed | Status::Disabled=> Style {
            background: Some(p.foreground.into()),
            text_color: p.error,
            border: border(Color { a: 0.5, ..p.error }),
            ..Default::default()
        },
        Status::Hovered => Style {
            background: Some(Color { a: 0.4, ..p.error }.into()),
            text_color: p.error,
            border: border(Color { a: 0.5, ..p.error }),
            ..Default::default()
        },
    }
}

pub fn hyperlink(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    match status {
        Status::Active | Status::Pressed | Status::Disabled=> Style {
            background: None,
            text_color: p.text,
            border: border(Color::TRANSPARENT),
            ..Default::default()
        },
        Status::Hovered => Style {
            background: None,
            text_color: p.accent,
            border: border(Color::TRANSPARENT),
            ..Default::default()
        },
    }
}

pub fn hyperlink_inverted(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();
    match status {
        Status::Active | Status::Pressed | Status::Disabled => Style {
            text_color: p.accent,
            ..hyperlink(theme, status)
        },
        Status::Hovered => Style {
            text_color: p.text,
            ..hyperlink(theme, status)
        },
    }
}

pub fn entry(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    match status {
        Status::Active | Status::Pressed | Status::Disabled=> Style {
            background: Some(p.foreground.into()),
            text_color: p.text,
            border: border(Color { a: 0.5, ..p.border }),
            ..Default::default()
        },
        Status::Hovered => Style {
            background: Some(Color { a: 0.4, ..p.accent }.into()),
            text_color: p.text,
            border: border(Color { a: 0.5, ..p.accent }),
            ..Default::default()
        },
    }
}

pub fn entry_error(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    match status {
        Status::Active | Status::Pressed | Status::Disabled => Style {
            background: Some(p.foreground.into()),
            text_color: p.text,
            border: border(Color { a: 0.5, ..p.error }),
            ..Default::default()
        },
        Status::Hovered => Style {
            background: Some(Color { a: 0.4, ..p.error }.into()),
            text_color: p.text,
            border: border(Color { a: 0.5, ..p.error }),
            ..Default::default()
        },
    }
}

pub fn start(theme: &Theme, status: Status) -> Style {
    let p = theme.palette();

    match status {
        Status::Active | Status::Pressed | Status::Disabled => Style {
            background: Some(p.foreground.into()),
            text_color: p.success,
            border: border(Color {
                a: 0.5,
                ..p.success
            }),
            ..Default::default()
        },
        Status::Hovered => Style {
            background: Some(
                Color {
                    a: 0.4,
                    ..p.success
                }
                .into(),
            ),
            text_color: p.text,
            border: border(Color {
                a: 0.5,
                ..p.success
            }),
            ..Default::default()
        },
    }
}
