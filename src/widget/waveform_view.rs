//! Simple Widget to view waveform
//!

mod marker;
mod style;
mod wave;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::keyboard::KeyCode;
use iced::mouse::Button;
use iced::window::Action;
use iced::{keyboard, BorderRadius, Color, Element, Length, Point, Rectangle};

pub use marker::Marker;
pub use style::{Appearance, StyleSheet};
pub use wave::WaveData;

use self::wave::Local;

const BAR_OVERLAP: f32 = 0.5;
const SCALE: f32 = 1.1;

pub struct WaveformViewer<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    wave: Option<&'a WaveData>,
    markers: Option<Vec<Marker>>,
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

    pub fn markers<I>(mut self, markers: I) -> Self
    where
        I: IntoIterator<Item = Marker>,
    {
        self.markers.get_or_insert(Vec::new()).extend(markers);
        self
    }

    pub fn markers_maybe<I>(self, markers: Option<I>) -> Self
    where
        I: IntoIterator<Item = Marker>,
    {
        match markers {
            Some(markers) => self.markers(markers),
            None => self,
        }
    }

    pub fn marker(mut self, marker: Marker) -> Self {
        self.markers.get_or_insert(Vec::new()).push(marker);
        self
    }

    pub fn marker_maybe(self, marker: Option<Marker>) -> Self {
        match marker {
            Some(marker) => self.marker(marker),
            None => self,
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
        self.on_cursor_click = Some(Box::new(callback));
        self
    }

    fn get_wave(&'a self, state: &'a State) -> Option<&'a WaveData>
    where
        Message: 'a,
    {
        match state.interpolated.as_ref() {
            Some(wave) => Some(wave),
            None => self.wave,
        }
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
    interpolated: Option<WaveData>,
    wave_id: u64,
}

impl State {
    fn new() -> Self {
        Self {
            zoom: 1.0,
            ..Default::default()
        }
    }

    fn reset_zoom(&mut self) {
        self.zoom = 1.0;
        self.interpolated = None;
    }

    fn zoom_wave(&mut self, wave: &WaveData) {
        const MAX: f32 = 10.0;
        self.zoom = self.zoom.clamp(0.05, 10.0);

        if self.zoom == 1.0 {
            self.interpolated = None;
        } else {
            let interpolated = wave::zoom(wave, self.zoom);
            self.interpolated = Some(interpolated);
        }
    }
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
        widget::tree::State::new(State::new())
    }

    fn diff(&self, tree: &mut widget::Tree) {
        let state = tree.state.downcast_mut::<State>();

        let Some(wave) = self.wave else {
            state.interpolated = None;
            state.wave_id = 0;
            state.zoom = 1.0;
            return;
        };

        if state.interpolated.is_some() && state.wave_id != wave.id() {
            state.zoom_wave(wave);
            state.wave_id = wave.id();
        }
    }

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
                    if let Some(wave) = self.get_wave(state) {
                        if state.dragging {
                            let current_cursor_x = position.x;
                            let start_offset = state.drag_start_offset.x;
                            let previous_offset = state.previous_offset as f32;

                            let new_offset = (start_offset - (current_cursor_x - previous_offset)) as usize;

                            let wave = &wave.peaks()[0];

                            state.wave_offset = wave.len().saturating_sub(1).min(new_offset);
                        }
                    }

                    iced::event::Status::Captured
                }

                iced::mouse::Event::WheelScrolled { delta } if cursor_in_bounds() => {
                    let mut zoom_wave = |y: f32| match self.wave {
                        Some(wave) => {
                            match y > 0.0 {
                                true => state.zoom *= SCALE,
                                false => state.zoom /= SCALE,
                            };
                            state.zoom_wave(wave);
                            iced::event::Status::Captured
                        }
                        None => iced::event::Status::Ignored,
                    };

                    match delta {
                        iced::mouse::ScrollDelta::Lines { y, .. } => zoom_wave(y),
                        iced::mouse::ScrollDelta::Pixels { y, .. } => zoom_wave(y),
                    }
                }
                _ => iced::event::Status::Ignored,
            },
            iced::Event::Keyboard(keyboard::Event::KeyReleased { key_code, .. }) => match key_code {
                KeyCode::Up => match self.wave {
                    Some(wave) => {
                        state.zoom *= SCALE;
                        state.zoom_wave(wave);
                        iced::event::Status::Captured
                    }
                    None => iced::event::Status::Ignored,
                },
                KeyCode::Down => match self.wave {
                    Some(wave) => {
                        state.zoom /= SCALE;
                        state.zoom_wave(wave);
                        iced::event::Status::Captured
                    }
                    None => iced::event::Status::Ignored,
                },
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
        if let Some(waveform) = self.get_wave(state) {
            let wave = &waveform.peaks()[0];
            let wave_offset = state.wave_offset.min(wave.len().saturating_sub(1));

            for offset in wave_offset..wave.len() {
                let wave_maxima = (layout_height / 2.0) * wave[offset].maxima;
                let wave_minima = (layout_height / 2.0) * wave[offset].minima.abs();

                let x = layout.bounds().x + offset as f32 - wave_offset as f32;

                if !layout.bounds().contains([x + BAR_OVERLAP, dc_offset.y].into()) {
                    break;
                }

                // Draw both top & bottom
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x,
                            y: dc_offset.y - wave_maxima,
                            width: 1.0 + BAR_OVERLAP,
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
            if let Some(markers) = &self.markers {
                let wave_width = wave.len() as f32;

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
