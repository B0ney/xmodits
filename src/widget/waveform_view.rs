//! Simple Widget to view waveform
//!

mod marker;
mod style;
mod wave;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::mouse::Button;
use iced::window::Action;
use iced::{BorderRadius, Color, Element, Length, Point, Rectangle};

pub use marker::Marker;
pub use style::{Appearance, StyleSheet};
pub use wave::WaveData;

const BAR_WIDTH: f32 = 1.0;
const BAR_OVERLAP: f32 = 0.5;

pub struct WaveformViewer<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    wave: Option<&'a WaveData>,
    markers: Option<&'a [Marker]>,
    width: Length,
    height: Length,
    on_cursor_click: Option<Box<dyn Fn(f32) -> Message + 'a>>,
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
            markers: None,
            width: Length::Fill,
            height: Length::Fill,
            on_cursor_click: None,
            style: Default::default(),
        }
    }

    pub fn with_markers(mut self, marker: &'a [Marker]) -> Self {
        self.markers = Some(marker);
        self
    }

    pub fn style(mut self, style: <Renderer::Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }

    pub fn on_cursor_drag<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        self.on_cursor_click = Some(Box::new(callback));
        self
    }
}

// Internal state of the widget
#[derive(Debug, Default)]
struct State {
    mouse_down: bool,
    dragging: bool,
    drag_start_offset: Point<f32>,
    previous_offset: usize,
    wave_offset: usize,
    zoom: f32,
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
        widget::tree::State::new(State::default())
    }

    // fn mouse_interaction(
    //     &self,
    //     _state: &widget::Tree,
    //     _layout: Layout<'_>,
    //     _cursor: iced::advanced::mouse::Cursor,
    //     _viewport: &iced::Rectangle,
    //     _renderer: &Renderer,
    // ) -> iced::advanced::mouse::Interaction {
    //     // _cursor.
    // }

    fn on_event(
        &mut self,
        _state: &mut widget::Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        let state = _state.state.downcast_mut::<State>();

        let cursor_in_bounds = || cursor.is_over(layout.bounds());

        match event {
            iced::Event::Mouse(mouse) => match mouse {
                iced::mouse::Event::ButtonPressed(Button::Left) if cursor_in_bounds() => {
                    state.mouse_down = true;
                    iced::event::Status::Captured
                }
                iced::mouse::Event::ButtonPressed(Button::Middle) if cursor_in_bounds() => {
                    state.dragging = true;
                    if let Some(pos) = cursor.position() {
                        state.drag_start_offset = pos
                    }
                    iced::event::Status::Captured
                }

                iced::mouse::Event::ButtonReleased(Button::Left) => match &self.on_cursor_click {
                    Some(callback) if cursor_in_bounds() => {
                        state.mouse_down = false;
                        shell.publish(callback(0.0));
                        iced::event::Status::Captured
                    }
                    _ => {
                        state.mouse_down = false;
                        iced::event::Status::Captured
                    }
                },

                iced::mouse::Event::ButtonReleased(Button::Middle) => {
                    state.dragging = false;
                    state.previous_offset = state.wave_offset;
                    iced::event::Status::Captured
                }

                iced::mouse::Event::CursorMoved { position } => {
                    if let Some(wave) = self.wave {
                        if state.dragging {
                            let current_cursor_x = position.x;
                            let start_offset = state.drag_start_offset.x;
                            let previous_offset = state.previous_offset as f32;

                            let new_offset = (start_offset - (current_cursor_x - previous_offset)) as usize;

                            let wave = &wave.0[0];

                            state.wave_offset = wave.len().saturating_sub(1).min(new_offset);
                        }
                    }

                    iced::event::Status::Captured
                }

                // iced::mouse::Event::WheelScrolled { delta } => {}
                _ => iced::event::Status::Ignored,
            },
            _ => iced::event::Status::Ignored,
        }
    }

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
        let state = tree.state.downcast_ref::<State>();

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
            x: layout.bounds().x + (layout_width / 2.0),
            y: layout.bounds().y + (layout_height / 2.0),
        };

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

        // Draw waveform
        if let Some(waveform) = self.wave {
            let wave = &waveform.0[0];
            let wave_offset = state.wave_offset.min(wave.len().saturating_sub(1));

            for offset in wave_offset..wave.len() {
                let wave_maxima = ((layout_height * 0.90) / 2.0) * wave[offset].maxima;
                let wave_minima = ((layout_height * 0.90) / 2.0) * wave[offset].minima.abs();

                let x = layout.bounds().x + offset as f32 * BAR_WIDTH - wave_offset as f32;

                if !layout.bounds().contains([x + BAR_OVERLAP, dc_offset.y].into()) {
                    break;
                }

                // Draw both top & bottom
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x,
                            y: dc_offset.y - wave_maxima,
                            width: BAR_WIDTH + BAR_OVERLAP,
                            height: wave_maxima + wave_minima,
                        },
                        border_radius: 0.0.into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                    appearance.wave_color,
                );
            }

            // Draw markers - only do so if we're rendering the waveform
            if let Some(markers) = self.markers {
                let wave_width = wave.len() as f32 * BAR_WIDTH;

                for marker in markers {
                    let x = layout.bounds().x + wave_width * marker.0 - wave_offset as f32;

                    if !layout.bounds().contains([x, dc_offset.y].into()) {
                        continue;
                    }

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
        }

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
                    if state.mouse_down {
                        appearance.wave_color
                    } else {
                        appearance.cursor_color
                    },
                );
            }
        }
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
