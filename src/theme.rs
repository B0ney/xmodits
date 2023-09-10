use iced::widget::{
    button, checkbox, container, pick_list, progress_bar, radio, rule, scrollable, slider, text,
    text_input, vertical_slider,
};
use iced::{application, overlay};


pub struct Theme(data::Theme);

impl Default for Theme {
    fn default() -> Self {
        Self(todo!())
    }
}

/* Widget styling implementations. Keep in alphabetical order. */

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> application::Appearance {
        todo!()
    }
}

impl button::StyleSheet for Theme {
    type Style = ();

    fn active(&self, style: &Self::Style) -> button::Appearance {
        todo!()
    }
}

impl checkbox::StyleSheet for Theme {
    type Style = ();

    fn active(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        todo!()
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        todo!()
    }
}

impl container::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        todo!()
    }
}

impl overlay::menu::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> overlay::menu::Appearance {
        todo!()
    }
}

impl pick_list::StyleSheet for Theme {
    type Style = ();

    fn active(&self, style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        todo!()
    }

    fn hovered(&self, style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        todo!()
    }
}

impl progress_bar::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> progress_bar::Appearance {
        todo!()
    }
}

impl radio::StyleSheet for Theme {
    type Style = ();

    fn active(&self, style: &Self::Style, is_selected: bool) -> radio::Appearance {
        todo!()
    }

    fn hovered(&self, style: &Self::Style, is_selected: bool) -> radio::Appearance {
        todo!()
    }
}

impl rule::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> rule::Appearance {
        todo!()
    }
}

impl scrollable::StyleSheet for Theme {
    type Style = ();

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        todo!()
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> scrollable::Scrollbar {
        todo!()
    }
}

impl slider::StyleSheet for Theme {
    type Style = ();

    fn active(&self, style: &Self::Style) -> vertical_slider::Appearance {
        todo!()
    }

    fn hovered(&self, style: &Self::Style) -> vertical_slider::Appearance {
        todo!()
    }

    fn dragging(&self, style: &Self::Style) -> vertical_slider::Appearance {
        todo!()
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        todo!()
    }
}

impl text_input::StyleSheet for Theme {
    type Style = ();

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        todo!()
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        todo!()
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        todo!()
    }

    fn value_color(&self, style: &Self::Style) -> iced::Color {
        todo!()
    }

    fn disabled_color(&self, style: &Self::Style) -> iced::Color {
        todo!()
    }

    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        todo!()
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        todo!()
    }
}

// impl vertical_slider::StyleSheet for Theme {
//     type Style = ();

//     fn active(&self, style: &Self::Style) -> vertical_slider::Appearance {
//         todo!()
//     }

//     fn hovered(&self, style: &Self::Style) -> vertical_slider::Appearance {
//         todo!()
//     }

//     fn dragging(&self, style: &Self::Style) -> vertical_slider::Appearance {
//         todo!()
//     }
// }
