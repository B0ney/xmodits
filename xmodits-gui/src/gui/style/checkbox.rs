use super::ColorPalette;
use iced::widget::checkbox;

#[derive(Default, Debug, Clone, Copy)]
pub enum CheckBox {
    #[default]
    Normal,
    Inverted,
}

impl checkbox::StyleSheet for ColorPalette {
    type Style = CheckBox;

    fn active(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let default = checkbox::Appearance {
            background: self.base.background.into(),
            icon_color: self.bright.primary,
            border_radius: 5.0,
            border_width: 1.2,
            border_color: self.base.border,
            text_color: Some(self.bright.surface),
        };
        match style {
            CheckBox::Normal => default,
            CheckBox::Inverted => checkbox::Appearance {
                background: self.base.foreground.into(),
                ..default
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let from_appearance = checkbox::Appearance {
            background: self.base.background.into(),
            icon_color: self.bright.primary,
            border_radius: 5.0,
            border_width: 2.0,
            border_color: self.bright.primary,
            text_color: Some(self.bright.surface),
        };

        match style {
            CheckBox::Normal => from_appearance,
            CheckBox::Inverted => checkbox::Appearance {
                background: self.base.foreground.into(),
                ..from_appearance
            },
        }
    }
}
