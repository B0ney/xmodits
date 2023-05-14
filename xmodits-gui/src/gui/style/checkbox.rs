use super::Theme;
use iced::widget::checkbox;

#[derive(Default, Debug, Clone, Copy)]
pub enum CheckBox {
    #[default]
    Normal,
    Inverted,
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckBox;

    fn active(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let default = checkbox::Appearance {
            background: self.palette().base.background.into(),
            icon_color: self.palette().bright.primary,
            border_radius: 5.0,
            border_width: 1.2,
            border_color: self.palette().base.border,
            text_color: Some(self.palette().bright.surface),
        };
        match style {
            CheckBox::Normal => default,
            CheckBox::Inverted => checkbox::Appearance {
                background: self.palette().base.foreground.into(),
                ..default
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let from_appearance = checkbox::Appearance {
            background: self.palette().base.background.into(),
            icon_color: self.palette().bright.primary,
            border_radius: 5.0,
            border_width: 2.0,
            border_color: self.palette().bright.primary,
            text_color: Some(self.palette().bright.surface),
        };

        match style {
            CheckBox::Normal => from_appearance,
            CheckBox::Inverted => checkbox::Appearance {
                background: self.palette().base.foreground.into(),
                ..from_appearance
            },
        }
    }
}
