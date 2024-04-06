use iced::border::{Border, Radius};
use iced::widget::container::{Catalog, Style, StyleFn};
use iced::{color, Color};

use super::helpers::border;
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

pub fn frame(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.palette().text),
        background: Some(theme.palette().foreground.into()),
        border: border(theme.palette().border),
        ..Default::default()
    }
}

pub fn black(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.palette().text),
        background: Some(theme.palette().background.into()),
        border: border(theme.palette().border),
        ..Default::default()
    }
}

pub fn black_hovered<'a>(hovered: bool) -> StyleFn<'a, Theme> {
    Box::new(move |theme| -> Style {
        if hovered {
            let p = theme.palette();
            Style {
                border: Border {
                    color: Color { a: 0.8, ..p.accent },
                    width: BORDER_WIDTH * 1.5,
                    radius: BORDER_RADIUS.into(),
                },
                ..black(theme)
            }
        } else {
            black(theme)
        }
    })
}

pub fn hovered<'a>(hovered: bool) -> StyleFn<'a, Theme> {
    Box::new(move |theme| -> Style {
        if hovered {
            let p = theme.palette();
            Style {
                border: Border {
                    color: Color { a: 0.8, ..p.accent },
                    width: BORDER_WIDTH * 1.5,
                    radius: BORDER_RADIUS.into(),
                },
                ..Default::default()
            }
        } else {
            Default::default()
        }
    })
}
