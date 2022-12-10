pub mod button;
pub mod checkbox;
mod radio;
pub mod scrollable;
pub mod text;
pub mod text_input;
use iced::overlay::menu;
use iced::widget::{container, pick_list, rule};

use iced::{application, color, Background, Color};
mod theme;
pub use theme::Theme;

#[derive(Default, Debug, Clone, Copy)]
pub enum Application {
    #[default]
    Default,
}

impl application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette().base.background,
            text_color: self.palette().bright.surface,
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

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Invisible => container::Appearance::default(),
            Container::Frame => container::Appearance {
                background: Some(Background::Color(self.palette().base.foreground)),
                text_color: Some(self.palette().bright.surface),
                border_color: color!(0x474747),
                border_radius: 5.0,
                border_width: 1.2,
                // ..container::Appearance::default()
            },
            Container::Black => container::Appearance {
                background: Some(Background::Color(color!(0x151515))),
                text_color: Some(self.palette().bright.surface),
                border_radius: 5.0,
                border_width: 1.2,
                border_color: color!(0x474747),
                // ..container::Appearance::default()

                // border_color: self.palette().normal.error,
            },
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum PickList {
    #[default]
    Default,
}

impl menu::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
        let p = self.palette();

        menu::Appearance {
            text_color: p.bright.surface,
            background: p.base.background.into(),
            border_width: 1.0,
            border_radius: 2.0,
            border_color: self.palette().normal.primary,
            selected_text_color: p.bright.surface,
            selected_background: p.normal.primary.into(),
        }
    }
}

impl pick_list::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &()) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: self.palette().bright.surface,
            background: self.palette().base.background.into(),
            border_width: 1.0,
            border_color: Color {
                // a: 0.5,
                ..self.palette().normal.primary
            },
            border_radius: 2.0,
            icon_size: 0.5,
            placeholder_color: self.palette().bright.surface,
        }
    }

    fn hovered(&self, style: &()) -> pick_list::Appearance {
        let active = self.active(style);
        pick_list::Appearance {
            border_color: self.palette().normal.primary,
            ..active
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Rule {
    #[default]
    Default,
}

impl rule::StyleSheet for Theme {
    type Style = Rule;

    fn appearance(&self, style: &Self::Style) -> rule::Appearance {
        match style {
            Rule::Default => rule::Appearance {
                color: self.palette().bright.surface,
                width: 2,
                radius: 2.0,
                fill_mode: rule::FillMode::Full,
            },
        }
    }
}
