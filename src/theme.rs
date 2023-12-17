use iced::widget::{
    button, checkbox, container, pick_list, progress_bar, radio, rule, scrollable, slider, text, text_input,
    vertical_slider,
};
use iced::{application, overlay, Background, Color};

use data::theme;

const BORDER_RADIUS: f32 = 5.0;
const BORDER_WIDTH: f32 = 1.5;

#[derive(Default, Clone)]
pub struct Theme(pub theme::Palette);

impl Theme {
    pub fn inner(&self) -> &theme::Palette {
        &self.0
    }
    // pub fn load(&mut self, theme: data::Theme) {
    //     let _ =
    // }
}

/* Widget styling implementations. Keep in alphabetical order. */

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.inner().middleground,
            text_color: self.inner().text,
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
    Dark,
    MediaStart,
    MediaMiddle,
    MediaEnd,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let p = self.inner();

        let appearance = button::Appearance {
            border_width: BORDER_WIDTH,
            border_radius: BORDER_RADIUS.into(),
            ..button::Appearance::default()
        };

        let active_appearance = |bg: Option<Color>, mc| button::Appearance {
            background: Some(Background::Color(bg.unwrap_or(p.foreground))),
            border_color: Color { a: 0.5, ..mc },
            text_color: mc,
            ..appearance
        };

        match style {
            Button::Primary => active_appearance(None, p.accent),
            Button::Cancel => active_appearance(None, p.error),
            Button::Unavailable => active_appearance(None, p.error),
            Button::Entry => button::Appearance {
                background: Some(Background::Color(p.foreground)),
                text_color: p.text,
                border_radius: BORDER_RADIUS.into(),
                border_width: BORDER_WIDTH,
                border_color: p.border,
                ..appearance
            },
            Button::Hyperlink => button::Appearance {
                background: None,
                text_color: p.text,
                ..appearance
            },
            Button::Start => active_appearance(None, p.success),
            Button::Delete => active_appearance(None, p.error),
            Button::HyperlinkInverted => button::Appearance {
                background: None,
                text_color: p.accent,
                ..appearance
            },
            Button::Dark => button::Appearance {
                background: Some(p.background.into()),
                text_color: p.text,
                border_radius: BORDER_RADIUS.into(),
                border_color: p.middleground,
                ..appearance
            },
            Button::MediaStart => button::Appearance {
                border_radius: [8.0, BORDER_RADIUS, BORDER_RADIUS, 8.0].into(),
                ..active_appearance(None, p.accent)
            },
            Button::MediaMiddle => active_appearance(None, p.accent),
            Button::MediaEnd => button::Appearance {
                border_radius: [BORDER_RADIUS, 8.0, 8.0, BORDER_RADIUS].into(),
                ..active_appearance(None, p.accent)
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self.inner();

        let hover_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.4, ..bg })),

            text_color: tc.unwrap_or(bg),
            ..active
        };

        match style {
            Button::Primary => hover_appearance(p.accent, Some(p.text)),
            Button::Unavailable => hover_appearance(p.error, None),
            Button::Entry => button::Appearance {
                border_color: Color { a: 0.5, ..p.accent },
                ..hover_appearance(p.accent, Some(p.text))
            },
            Button::Hyperlink => button::Appearance {
                background: None,
                ..hover_appearance(p.accent, None)
            },
            Button::Start => hover_appearance(p.success, Some(p.text)),
            Button::Delete => hover_appearance(p.error, Some(p.text)),
            Button::HyperlinkInverted => button::Appearance {
                background: None,
                text_color: p.text,
                ..hover_appearance(p.accent, None)
            },
            Button::Cancel => hover_appearance(p.error, Some(p.text)),
            Button::Dark => button::Appearance {
                background: Some(p.background.into()),
                text_color: p.accent,
                ..active
            },
            Button::MediaStart => button::Appearance {
                border_radius: [8.0, BORDER_RADIUS, BORDER_RADIUS, 8.0].into(),
                ..hover_appearance(p.accent, Some(p.text))
            },
            Button::MediaMiddle => hover_appearance(p.accent, Some(p.text)),
            Button::MediaEnd => button::Appearance {
                border_radius: [BORDER_RADIUS, 8.0, 8.0, BORDER_RADIUS].into(),
                ..hover_appearance(p.accent, Some(p.text))
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        match style {
            _ => button::Appearance { ..active },
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance { ..self.active(style) }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum CheckBox {
    #[default]
    Normal,
    Inverted,
    Entry,
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckBox;

    fn active(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let p = self.inner();

        let default = checkbox::Appearance {
            background: p.middleground.into(),
            icon_color: p.accent,
            border_radius: BORDER_RADIUS.into(),
            border_width: BORDER_WIDTH,
            border_color: p.border,
            text_color: Some(p.text),
        };
        match style {
            CheckBox::Normal => default,
            CheckBox::Inverted => checkbox::Appearance {
                background: p.foreground.into(),
                ..default
            },
            CheckBox::Entry => checkbox::Appearance {
                // border_color: Color { a: 0.25, ..p.accent },
                ..default
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let p = self.inner();

        let default = checkbox::Appearance {
            background: p.middleground.into(),
            icon_color: p.accent,
            border_radius: BORDER_RADIUS.into(),
            border_width: 2.0,
            border_color: p.accent,
            text_color: Some(p.text),
        };

        match style {
            CheckBox::Normal => default,
            CheckBox::Inverted => checkbox::Appearance {
                background: p.foreground.into(),
                ..default
            },
            CheckBox::Entry => default,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Container {
    #[default]
    Invisible,
    Frame,
    Black,
    BlackHovered(bool),
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let dark = container::Appearance {
            background: Some(self.inner().background.into()),
            text_color: Some(self.inner().text),
            border_radius: BORDER_RADIUS.into(),
            border_width: BORDER_WIDTH,
            border_color: self.inner().border,
        };
        match style {
            Container::Invisible => container::Appearance::default(),
            Container::Frame => container::Appearance {
                background: Some(Background::Color(self.inner().foreground)),
                text_color: Some(self.inner().text),
                border_color: self.inner().border,
                border_radius: BORDER_RADIUS.into(),
                border_width: BORDER_WIDTH,
            },
            Container::Black => dark,
            Container::BlackHovered(hovered) => match hovered {
                true => container::Appearance {
                    border_color: Color {
                        a: 0.80,
                        ..self.inner().accent
                    },
                    border_width: 2.0,
                    ..dark
                },
                false => dark,
            },
        }
    }
}

// impl iced_aw::date_picker::StyleSheet for Theme {
//     type Style = ();

//     fn active(&self, _style: &Self::Style) -> iced_aw::date_picker::Appearance {
//         let p = self.inner();
//         iced_aw::date_picker::Appearance {
//             background: Background::Color(p.background),
//             border_radius: BORDER_RADIUS,
//             border_width: BORDER_WIDTH,
//             border_color: p.border,
//             text_color: p.text,
//             text_attenuated_color: p.background,
//             day_background: Background::Color(p.foreground),
//         }
//     }

//     fn selected(&self, style: &Self::Style) -> iced_aw::date_picker::Appearance {
//         let p = self.inner();

//         iced_aw::date_picker::Appearance {
//             border_color: p.accent,
//             day_background: Background::Color(Color { a: 0.75, ..p.accent }),
//             ..self.active(style)
//         }
//     }

//     fn hovered(&self, style: &Self::Style) -> iced_aw::date_picker::Appearance {
//         let p = self.inner();

//         iced_aw::date_picker::Appearance {
//             day_background: Background::Color(Color { a: 0.4, ..p.accent }),
//             ..self.active(style)
//         }
//     }

//     fn focused(&self, style: &Self::Style) -> iced_aw::date_picker::Appearance {
//         self.active(style)
//     }
// }

impl overlay::menu::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> overlay::menu::Appearance {
        overlay::menu::Appearance {
            text_color: self.inner().text,
            background: self.inner().middleground.into(),
            border_width: BORDER_WIDTH,
            border_radius: BORDER_RADIUS.into(),
            border_color: self.inner().border,
            selected_text_color: self.inner().text,
            selected_background: Color {
                a: 0.25,
                ..self.inner().accent
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
            text_color: p.text,
            background: p.middleground.into(),
            border_width: BORDER_WIDTH,
            border_color: p.border,
            border_radius: BORDER_RADIUS.into(),
            handle_color: p.text,
            placeholder_color: p.text,
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        let active = self.active(style);
        pick_list::Appearance {
            border_color: self.inner().accent,
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
            background: Background::Color(p.middleground),
            bar: Background::Color(p.accent),
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
            dot_color: p.accent,
            border_width: BORDER_WIDTH,
            border_color: p.accent,
            text_color: None,
        }
    }

    fn hovered(&self, style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        let active = self.active(style, true);
        let p = self.inner();

        radio::Appearance {
            dot_color: p.accent,
            border_color: p.accent,
            border_width: 2.0,
            ..active
        }
    }
}

impl rule::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: self.inner().border,
            width: 1,
            radius: 1.0.into(),
            fill_mode: rule::FillMode::Full,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Scrollable {
    #[default]
    Normal,
    Dark,
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        let p = self.inner();

        let from_appearance = |c: Color, d: Color| scrollable::Scrollbar {
            background: Some(Background::Color(c)),
            border_radius: BORDER_RADIUS.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: d,
                border_radius: 3.0.into(),
                border_width: BORDER_WIDTH,
                border_color: p.border,
            },
        };

        let color = (p.middleground, p.foreground);

        match style {
            Scrollable::Normal => from_appearance(color.0, color.1),
            Scrollable::Dark => from_appearance(color.1, color.0),
        }
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> scrollable::Scrollbar {
        let p = self.inner();

        scrollable::Scrollbar {
            scroller: scrollable::Scroller {
                color: if is_mouse_over_scrollbar {
                    Color { a: 0.5, ..p.accent }
                } else {
                    self.active(style).scroller.color
                },
                border_color: if is_mouse_over_scrollbar {
                    Color { a: 0.75, ..p.accent }
                } else {
                    self.active(style).scroller.border_color
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
                colors: (p.accent, p.accent),
                width: 3.0,
                border_radius: Default::default(),
            },
            handle: vertical_slider::Handle {
                shape: vertical_slider::HandleShape::Circle { radius: 3.0 },
                color: p.accent,
                border_width: 3.0,
                border_color: p.accent,
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
    Warning,
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
            Text::Error => text::Appearance { color: Some(p.error) },
            Text::Warning => text::Appearance {
                color: Some(p.warning),
            },
            Text::Color(c) => text::Appearance { color: Some(c) },
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum TextInputStyle {
    #[default]
    Normal,
    Inverted,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInputStyle;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let p = self.inner();

        let default = text_input::Appearance {
            background: Background::Color(p.foreground),
            border_radius: BORDER_RADIUS.into(),
            border_width: BORDER_WIDTH,
            border_color: p.border,
            icon_color: p.foreground,
        };

        match style {
            TextInputStyle::Normal => default,
            TextInputStyle::Inverted => text_input::Appearance {
                background: p.middleground.into(),
                border_radius: BORDER_RADIUS.into(),
                ..default
            },
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let p = self.inner();

        text_input::Appearance {
            border_color: p.accent,
            ..self.active(style)
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        self.inner().text
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        self.inner().accent
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        self.inner().text
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        Color {
            a: 0.5,
            ..self.inner().accent
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}
