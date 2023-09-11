use iced::widget::{
    button, checkbox, container, pick_list, progress_bar, radio, rule, scrollable, slider, text,
    text_input, vertical_slider,
};
use iced::{application, overlay, Background, Color};

use data::theme;

#[derive(Default)]
pub struct Theme(theme::Palette);

impl Theme {
    pub fn inner(&self) -> &theme::Palette {
        &self.0
    }
}

/* Widget styling implementations. Keep in alphabetical order. */

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.inner().base.background,
            text_color: self.inner().bright.surface,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    #[default]
    Primary,
    Cancel,
    Hyperlink,
    HyperlinkInverted,
    Unavailable,
    Entry,
    Start,
    Delete,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let p = self.inner();

        let appearance = button::Appearance {
            border_width: 1.0,
            border_radius: 8.0.into(),
            ..button::Appearance::default()
        };

        let active_appearance = |bg: Option<Color>, mc| button::Appearance {
            background: Some(Background::Color(bg.unwrap_or(p.base.foreground))),
            border_color: Color { a: 0.5, ..mc },
            text_color: mc,
            ..appearance
        };

        match style {
            Button::Primary => active_appearance(None, p.bright.primary),
            Button::Cancel => active_appearance(None, p.bright.error),
            Button::Unavailable => active_appearance(None, p.bright.error),
            Button::Entry => button::Appearance {
                background: Some(Background::Color(p.base.foreground)),
                text_color: p.bright.surface,
                border_radius: 5.0.into(),
                border_width: 1.0,
                border_color: p.base.border,
                ..appearance
            },
            Button::Hyperlink => button::Appearance {
                background: None,
                text_color: p.bright.surface,
                ..appearance
            },
            Button::Start => active_appearance(None, p.bright.secondary),
            Button::Delete => active_appearance(None, p.bright.error),
            Button::HyperlinkInverted => button::Appearance {
                background: None,
                text_color: p.bright.primary,

                ..appearance
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self.inner();

        let hover_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.25, ..bg })),

            text_color: tc.unwrap_or(bg),
            ..active
        };

        match style {
            Button::Primary => hover_appearance(p.bright.primary, Some(p.bright.surface)),
            Button::Unavailable => hover_appearance(p.bright.error, None),
            Button::Entry => hover_appearance(p.bright.primary, Some(p.bright.surface)),
            Button::Hyperlink => button::Appearance {
                background: None,
                ..hover_appearance(p.bright.primary, None)
            },
            Button::Start => hover_appearance(p.bright.secondary, Some(p.bright.surface)),
            Button::Delete => hover_appearance(p.bright.error, Some(p.bright.surface)),
            Button::HyperlinkInverted => button::Appearance {
                background: None,
                text_color: p.bright.surface,
                ..hover_appearance(p.bright.primary, None)
            },
            Button::Cancel => hover_appearance(p.bright.error, Some(p.bright.surface)),
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        match style {
            _ => button::Appearance { ..active },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self.active(style)
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum CheckBox {
    #[default]
    Normal,
    Inverted,
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckBox;

    fn active(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let p = self.inner();

        let default = checkbox::Appearance {
            background: p.base.background.into(),
            icon_color: p.bright.primary,
            border_radius: 5.0.into(),
            border_width: 1.2,
            border_color: p.base.border,
            text_color: Some(p.bright.surface),
        };
        match style {
            CheckBox::Normal => default,
            CheckBox::Inverted => checkbox::Appearance {
                background: p.base.foreground.into(),
                ..default
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let p = self.inner();

        let from_appearance = checkbox::Appearance {
            background: p.base.background.into(),
            icon_color: p.bright.primary,
            border_radius: 5.0.into(),
            border_width: 2.0,
            border_color: p.bright.primary,
            text_color: Some(p.bright.surface),
        };

        match style {
            CheckBox::Normal => from_appearance,
            CheckBox::Inverted => checkbox::Appearance {
                background: p.base.foreground.into(),
                ..from_appearance
            },
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
                background: Some(Background::Color(self.inner().base.foreground)),
                text_color: Some(self.inner().bright.surface),
                border_color: self.inner().base.border,
                border_radius: 5.0.into(),
                border_width: 1.2,
            },
            Container::Black => container::Appearance {
                background: Some(self.inner().base.dark.into()),
                text_color: Some(self.inner().bright.surface),
                border_radius: 5.0.into(),
                border_width: 1.2,
                border_color: self.inner().base.border,
            },
        }
    }
}

impl overlay::menu::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> overlay::menu::Appearance {
        overlay::menu::Appearance {
            text_color: self.inner().bright.surface,
            background: self.inner().base.background.into(),
            border_width: 1.2,
            border_radius: 5.0.into(),
            border_color: self.inner().base.border,
            selected_text_color: self.inner().bright.surface,
            selected_background: Color {
                a: 0.25,
                ..self.inner().bright.primary
            }
            .into(),
        }
    }
}

impl pick_list::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
        let p = self.inner();

        pick_list::Appearance {
            text_color: p.bright.surface,
            background: p.base.background.into(),
            border_width: 1.2,
            border_color: p.base.border,
            border_radius: 5.0.into(),
            handle_color: p.bright.surface,
            placeholder_color: p.bright.surface,
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        let active = self.active(style);
        pick_list::Appearance {
            border_color: self.inner().bright.primary,
            border_width: 2.0,
            ..active
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum ProgressBar {
    #[default]
    Default,
    Disrupted,
}

impl progress_bar::StyleSheet for Theme {
    type Style = ProgressBar;

    fn appearance(&self, _style: &Self::Style) -> progress_bar::Appearance {
        let p = self.inner();

        progress_bar::Appearance {
            background: Background::Color(p.base.background),
            bar: Background::Color(p.normal.primary),
            border_radius: 64.0.into(),
        }
    }
}

impl radio::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        let p = self.inner();

        radio::Appearance {
            background: Color::TRANSPARENT.into(),
            dot_color: p.bright.primary,
            border_width: 1.0,
            border_color: p.bright.primary,
            text_color: None,
        }
    }

    fn hovered(&self, style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        let active = self.active(style, true);
        let p = self.inner();

        radio::Appearance {
            dot_color: p.bright.primary,
            border_color: p.bright.primary,
            border_width: 2.0,
            ..active
        }
    }
}

impl rule::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: self.inner().base.border,
            width: 1,
            radius: 1.0.into(),
            fill_mode: rule::FillMode::Full,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Scrollable {
    #[default]
    Description,
    Dark,
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        let p = self.inner();

        let from_appearance = |c: Color, d: Color| scrollable::Scrollbar {
            background: Some(Background::Color(c)),
            border_radius: 5.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: d,
                border_radius: 5.0.into(),
                border_width: 1.0,
                border_color: p.base.border,
            },
        };

        let color = (p.base.background, p.base.foreground);

        match style {
            Scrollable::Description => from_appearance(color.0, color.1),
            Scrollable::Dark => from_appearance(color.1, color.0),
        }
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> scrollable::Scrollbar {
        let p = self.inner();

        scrollable::Scrollbar {
            scroller: scrollable::Scroller {
                color: if is_mouse_over_scrollbar {
                    p.normal.primary
                } else {
                    self.active(style).scroller.color
                },
                ..self.active(style).scroller
            },
            ..self.active(style)
        }
    }

    fn dragging(&self, style: &Self::Style) -> scrollable::Scrollbar {
        let hovered = self.hovered(style, true);

        scrollable::Scrollbar {
            scroller: scrollable::Scroller { ..hovered.scroller },
            ..hovered
        }
    }
}

impl slider::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> vertical_slider::Appearance {
        let p = self.inner();

        vertical_slider::Appearance {
            rail: slider::Rail {
                colors: (p.normal.primary, p.normal.primary),
                width: 3.0,
                border_radius: Default::default(),
            },
            handle: vertical_slider::Handle {
                shape: vertical_slider::HandleShape::Circle { radius: 3.0 },
                color: p.normal.primary,
                border_width: 3.0,
                border_color: p.normal.primary,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> vertical_slider::Appearance {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> vertical_slider::Appearance {
        self.active(style)
    }
}

#[derive(Default, Clone, Copy)]
pub enum Text {
    #[default]
    Default,
    Error,
    Color(Color),
}

impl From<Color> for Text {
    fn from(color: Color) -> Self {
        Text::Color(color)
    }
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        let p = self.inner();

        match style {
            Text::Default => Default::default(),
            Text::Error => text::Appearance {
                color: Some(p.bright.error),
            },
            Text::Color(c) => text::Appearance { color: Some(c) },
        }
    }
}

impl text_input::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        let p = self.inner();

        text_input::Appearance {
            background: Background::Color(p.base.foreground),
            border_radius: 8.0.into(),
            border_width: 1.2,
            border_color: p.base.border,
            icon_color: p.base.foreground,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let p = self.inner();

        text_input::Appearance {
            border_color: p.bright.primary,
            ..self.active(style)
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        self.inner().normal.surface
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        self.inner().normal.primary
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        self.inner().normal.surface
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        self.inner().normal.primary
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}
