use iced::widget::checkbox;
use iced::{Background, Color, color};
use super::Theme;

#[derive(Default, Debug, Clone, Copy)]
pub enum CheckBox {
    #[default]
    PackageEnabled,
    PackageDisabled,
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckBox;

    fn active(&self, style: Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let default = checkbox::Appearance {
                background: Background::Color(self.palette().base.background),
                checkmark_color: self.palette().bright.primary,
                border_radius: 5.0,
                border_width: 1.2,
                border_color: color!(0x474747),
                text_color: Some(self.palette().bright.surface),
            };
        match style {
            CheckBox::PackageEnabled => default,
            CheckBox::PackageDisabled => checkbox::Appearance { 
                background: Background::Color(self.palette().base.foreground),
                ..default
            },
        }

        // match style {
        //     CheckBox::PackageEnabled => checkbox::Appearance {
        //         background: Background::Color(self.palette().base.background),
        //         checkmark_color: self.palette().bright.primary,
        //         border_radius: 5.0,
        //         border_width: 1.0,
        //         border_color: self.palette().base.background,
        //         text_color: Some(self.palette().bright.surface),
        //     },
        //     CheckBox::PackageDisabled => checkbox::Appearance {
        //         background: Background::Color(Color {
        //             a: 0.55,
        //             ..self.palette().base.background
        //         }),
        //         checkmark_color: self.palette().bright.primary,
        //         border_radius: 5.0,
        //         border_width: 1.0,
        //         border_color: self.palette().normal.primary,
        //         text_color: Some(self.palette().normal.primary),
        //     },
        //     CheckBox::SettingsEnabled => checkbox::Appearance {
        //         background: Background::Color(self.palette().base.background),
        //         checkmark_color: self.palette().bright.primary,
        //         border_radius: 5.0,
        //         border_width: 1.0,
        //         border_color: self.palette().bright.primary,
        //         text_color: Some(self.palette().bright.surface),
        //     },
        //     CheckBox::SettingsDisabled => checkbox::Appearance {
        //         background: Background::Color(self.palette().base.foreground),
        //         checkmark_color: self.palette().bright.primary,
        //         border_radius: 5.0,
        //         border_width: 1.0,
        //         border_color: self.palette().normal.primary,
        //         text_color: Some(self.palette().bright.surface),
        //     },
        // }
    }

    fn hovered(&self, style: Self::Style, is_checked: bool) -> checkbox::Appearance {
        let from_appearance = || checkbox::Appearance {
            background: Background::Color(self.palette().base.background),
            checkmark_color: self.palette().bright.primary,
            border_radius: 5.0,
            border_width: 2.0,
            border_color: self.palette().bright.primary,
            text_color: Some(self.palette().bright.surface),
        };
        
        match style {
            CheckBox::PackageEnabled => from_appearance(),
            // CheckBox::SettingsEnabled => from_appearance(),
            CheckBox::PackageDisabled => self.active(style, is_checked),
            // CheckBox::SettingsDisabled => self.active(style, is_checked),
        }
    }
}