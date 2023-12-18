//! todo display waveform and spectrogram
mod style;
mod wave;
mod cursor;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::window::Action;
use iced::{Length, Element};

pub use style::{Appearance, StyleSheet};
pub use wave::WaveData;
pub use cursor::Cursor;


pub struct Waveform<'a, Message, Renderer> 
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,    
{
    wave: &'a WaveData,
    // cursors: &'a [Cursor],
    width: Length,
    height: Length,
    on_cursor_drag: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    style: <Renderer::Theme as StyleSheet>::Style
}

impl<'a, Message, Renderer> Waveform<'a, Message, Renderer> 
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet, 
{
    pub fn new(wave: &'a WaveData) -> Self {
        Self {
            wave,
            // cursors,
            width: Length::Fill,
            height: Length::Fill,
            on_cursor_drag: None,
            style: Default::default(),
        }
    }

    pub fn on_cursor_drag<F>(mut self, callback: F) -> Self 
    where F: Fn(f32) -> Message + 'a,
    {
        self.on_cursor_drag = Some(Box::new(callback));
        self
    }
}

// Internal state of the widget
struct State {
    clicked: bool,
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Waveform<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn width(&self) -> iced::Length {
        self.width
    }

    fn height(&self) -> iced::Length {
        self.height
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> iced::advanced::layout::Node {
        // todo!()

        layout::Node::new(limits.max())
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State { clicked: false })
    }

    fn on_event(
        &mut self,
        state: &mut widget::Tree,
        _event: iced::Event,
        _layout: Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        _shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        let state = state.state.downcast_mut::<State>();
        todo!()
    }

    fn mouse_interaction(
        &self,
        _state: &widget::Tree,
        _layout: Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        todo!()
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        todo!()
    }

}

impl<'a, Message, Renderer>
    From<Waveform<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(
        wave_form: Waveform<'a, Message, Renderer>,
    ) -> Self {
        Self::new(wave_form)
    }
}