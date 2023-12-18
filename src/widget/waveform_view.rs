//! Simple Widget to view waveform
//!

mod cursor;
mod style;
mod wave;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::window::Action;
use iced::{BorderRadius, Color, Element, Length, Point, Rectangle};

pub use cursor::Cursor;
pub use style::{Appearance, StyleSheet};
pub use wave::WaveData;

pub struct WaveformViewer<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    wave: Option<&'a WaveData>,
    // cursors: &'a [Cursor],
    width: Length,
    height: Length,
    on_cursor_drag: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, Message, Renderer> WaveformViewer<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    pub fn new(wave: &'a WaveData) -> Self {
        Self::new_maybe(Some(wave))
    }

    pub fn new_maybe(wave: Option<&'a WaveData>) -> Self {
        Self {
            wave,
            width: Length::Fill,
            height: Length::Fill,
            on_cursor_drag: None,
            style: Default::default(),
        }
    }

    pub fn style(mut self, style: <Renderer::Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }

    pub fn on_cursor_drag<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        self.on_cursor_drag = Some(Box::new(callback));
        self
    }
}

// Internal state of the widget
struct State {
    clicked: bool,
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for WaveformViewer<'a, Message, Renderer>
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
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> iced::advanced::layout::Node {
        layout::Node::new(limits.max())
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State { clicked: false })
    }

    // fn mouse_interaction(
    //     &self,
    //     _state: &widget::Tree,
    //     _layout: Layout<'_>,
    //     _cursor: iced::advanced::mouse::Cursor,
    //     _viewport: &iced::Rectangle,
    //     _renderer: &Renderer,
    // ) -> iced::advanced::mouse::Interaction {
    //     todo!()
    // }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        // Draw background
        let appearance = theme.appearance(&self.style);      
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: appearance.border_radius,
                border_width: appearance.border_width,
                border_color: appearance.border_color,
            },
            appearance.background,
        );

        let layout_width = layout.bounds().width;
        let layout_height = layout.bounds().height;

        let dc_offset = Point {
            x: layout.bounds().x + (layout.bounds().width / 2.0),
            y: layout.bounds().y + (layout.bounds().height / 2.0),
        };

        // Draw waveform
        if let Some(waveform) = self.wave {
            let wave = &waveform.0[0];
            let bar_width = 1.0;
            let overlap = 0.5;

            for offset in 0..wave.len() {
                let wave_maxima = ((layout_height * 0.90) / 2.0) * wave[offset].maxima;
                let wave_minima = ((layout_height * 0.90) / 2.0) * wave[offset].minima.abs();

                let x = layout.bounds().x + offset as f32 * bar_width;

                if !layout.bounds().contains([x + overlap, dc_offset.y].into()) {
                    break;
                }

                // Draw both top & bottom
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x,
                            y: dc_offset.y - wave_maxima,
                            width: bar_width + overlap,
                            height: wave_maxima + wave_minima,
                        },
                        border_radius: 0.0.into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                    appearance.wave_color,
                );
            }
        }

        // Draw DC line
        let line_thickness = 1.5;
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: layout.bounds().x,
                    y: dc_offset.y - (line_thickness / 2.0),
                    width: layout_width,
                    height: line_thickness,
                },
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            appearance.wave_color,
        );

        // Draw cursor
        if cursor.is_over(layout.bounds()) {
            if let Some(Point { x, .. }) = cursor.position() {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x,
                            y: layout.bounds().y,
                            width: 2.0,
                            height: layout.bounds().height,
                        },
                        border_radius: 0.0.into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                    appearance.cursor_color,
                );
            }
        }

        // Draw loop points...

        // Draw play cursor
    }
}

impl<'a, Message, Renderer> From<WaveformViewer<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(wave_form: WaveformViewer<'a, Message, Renderer>) -> Self {
        Self::new(wave_form)
    }
}
