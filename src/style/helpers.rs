use iced::{Border, Color};
use super::{BORDER_RADIUS, BORDER_WIDTH};

pub fn border(color: Color) -> Border {
    Border {
        color,
        width: BORDER_WIDTH,
        radius: BORDER_RADIUS.into(),
    }
}