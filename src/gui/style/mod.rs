pub mod button;
pub mod checkbox;
pub mod progress_bar;
pub mod radio;
pub mod scrollable;
pub mod slider;
pub mod svg;
pub mod text;
pub mod text_input;
// pub mod rule;
use iced::overlay::menu;
use iced::widget::{container, pick_list, rule};

use iced::{application, Background, Color};
mod theme;
pub use theme::Themes;
pub type Theme = ColorPalette;
pub use theme::ColorPalette;

#[derive(Default, Debug, Clone, Copy)]
pub enum Application {
    #[default]
    Default,
}

impl application::StyleSheet for ColorPalette {
    type Style = Application;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.base.background,
            text_color: self.bright.surface,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Container {
    #[default]
    Invisible,
    Frame,
    Black,
}

impl container::StyleSheet for ColorPalette {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Invisible => container::Appearance::default(),
            Container::Frame => container::Appearance {
                background: Some(Background::Color(self.base.foreground)),
                text_color: Some(self.bright.surface),
                border_color: self.base.border,
                border_radius: 5.0.into(),
                border_width: 1.2,
                // ..container::Appearance::default()
            },
            Container::Black => container::Appearance {
                background: Some(self.base.dark.into()),
                text_color: Some(self.bright.surface),
                border_radius: 5.0.into(),
                border_width: 1.2,
                border_color: self.base.border,
                // ..container::Appearance::default()

                // border_color: self.normal.error,
            },
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum PickList {
    #[default]
    Default,
}

impl menu::StyleSheet for ColorPalette {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
        let p = self;

        menu::Appearance {
            text_color: p.bright.surface,
            background: p.base.background.into(),
            border_width: 1.2,
            border_radius: 5.0.into(),
            border_color: self.base.border,
            selected_text_color: p.bright.surface,
            selected_background: Color {
                a: 0.25,
                ..p.bright.primary
            }
            .into(),
        }
    }
}

impl pick_list::StyleSheet for ColorPalette {
    type Style = ();

    fn active(&self, _style: &()) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: self.bright.surface,
            background: self.base.background.into(),
            border_width: 1.2,
            border_color: self.base.border,
            border_radius: 5.0.into(),
            handle_color: self.bright.surface,
            placeholder_color: self.bright.surface,
        }
    }

    fn hovered(&self, style: &()) -> pick_list::Appearance {
        let active = self.active(style);
        pick_list::Appearance {
            border_color: self.bright.primary,
            border_width: 2.0,
            ..active
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Rule {
    #[default]
    Default,
}

impl rule::StyleSheet for ColorPalette {
    type Style = Rule;

    fn appearance(&self, style: &Self::Style) -> rule::Appearance {
        match style {
            Rule::Default => rule::Appearance {
                color: self.base.border,
                width: 1,
                radius: 1.0.into(),
                fill_mode: rule::FillMode::Full,
            },
        }
    }
}
