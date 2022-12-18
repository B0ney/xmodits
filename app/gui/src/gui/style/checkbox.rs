use super::Theme;
use iced::widget::checkbox;
use iced::{color, Background};

#[derive(Default, Debug, Clone, Copy)]
pub enum CheckBox {
    #[default]
    Enabled,
    Disabled,
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckBox;

    fn active(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let default = checkbox::Appearance {
            background: Background::Color(self.palette().base.background),
            checkmark_color: self.palette().bright.primary,
            border_radius: 5.0,
            border_width: 1.2,
            border_color: color!(0x474747),
            text_color: Some(self.palette().bright.surface),
        };
        match style {
            CheckBox::Enabled => default,
            CheckBox::Disabled => checkbox::Appearance {
                background: Background::Color(self.palette().base.foreground),
                ..default
            },
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        let from_appearance = || checkbox::Appearance {
            background: Background::Color(self.palette().base.background),
            checkmark_color: self.palette().bright.primary,
            border_radius: 5.0,
            border_width: 2.0,
            border_color: self.palette().bright.primary,
            text_color: Some(self.palette().bright.surface),
        };

        match style {
            CheckBox::Enabled => from_appearance(),
            CheckBox::Disabled => self.active(style, is_checked),
        }
    }
}
