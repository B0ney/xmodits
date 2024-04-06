use iced::border::{Border, Radius};
use iced::widget::container;
use iced::widget::scrollable::{Catalog, Scrollbar, Scroller, Status, Style, StyleFn};
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
    let p = theme.palette();
    match status {
        Status::Active => {
            let scrollbar = Scrollbar {
                background: Some(p.middleground.into()),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 3.0.into(),
                },
                scroller: Scroller {
                    color: p.foreground,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 3.0.into(),
                    },
                },
            };

            Style {
                container: container::Style::default(),
                vertical_scrollbar: scrollbar,
                horizontal_scrollbar: scrollbar,
                gap: None,
            }
        }
        Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
        } => {
            let scrollbar_hovered = is_horizontal_scrollbar_hovered | is_vertical_scrollbar_hovered;

            let scrollbar = Scrollbar {
                background: Some(p.middleground.into()),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 3.0.into(),
                },
                scroller: Scroller {
                    color: if scrollbar_hovered {
                        Color { a: 0.5, ..p.accent }
                    } else {
                        Color::TRANSPARENT
                    },
                    border: Border {
                        color: if scrollbar_hovered {
                            Color {
                                a: 0.75,
                                ..p.accent
                            }
                        } else {
                            Color::TRANSPARENT
                        },
                        width: BORDER_WIDTH,
                        radius: 3.0.into(),
                    },
                },
            };

            Style {
                container: container::Style::default(),
                vertical_scrollbar: scrollbar,
                horizontal_scrollbar: scrollbar,
                gap: None,
            }
        }
        Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
        } => primary(
            theme,
            Status::Hovered {
                is_horizontal_scrollbar_hovered: is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_hovered: is_vertical_scrollbar_dragged,
            },
        ),
    }
}
