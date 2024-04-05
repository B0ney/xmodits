use iced::border::{Border, Radius};
use iced::widget::overlay::menu::{Catalog, Style, StyleFn};
use iced::{color, Color};

use super::{Theme, BORDER_RADIUS, BORDER_WIDTH};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;
    
    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        todo!()
    }
    
    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
        todo!()
    }

   
}

pub fn primary(theme: &Theme) -> Style {
    todo!()
}
