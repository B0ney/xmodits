//! Simple Widget to view waveform

mod marker;
mod style;
mod wave;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Renderer as _};
use iced::advanced::widget::{self, Widget};
use iced::keyboard::KeyCode;
use iced::mouse::Button;
use iced::widget::canvas::{self, Renderer as _};
use iced::{keyboard, Color, Element, Length, Point, Rectangle, Renderer, Vector, Size};
use std::cell::RefCell;

pub use marker::Marker;
pub use style::{Appearance, StyleSheet};
pub use wave::{Local, WaveData};

const SCALE: f32 = 1.2;
const MAX_SCALE: f32 = 10.0;
const MIN_SCALE: f32 = 0.02;

pub struct WaveformViewer<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    wave: Option<&'a WaveData>,
    markers: Option<Vec<Marker>>,
    width: Length,
    height: Length,
    on_cursor_click: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    style: Theme::Style,
}

impl<'a, Message, Theme> WaveformViewer<'a, Message, Theme>
where
    Theme: StyleSheet,
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

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
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

    pub fn style(mut self, style: Theme::Style) -> Self {
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

#[derive(Debug, Default)]
enum WaveGeometry {
    /// No geometry has been constructed yet.
    #[default]
    None,
    /// 1:1 representation of a waveform.
    Original(canvas::Path),
    /// Scaled representation of a waveform.
    /// Used when rendering the original waveform is expensive.
    /// Uses linear interpolation to effectively shrink the original wave data,
    /// so less points are used.
    Scaled { scale: f32, wave: canvas::Path },
}

impl WaveGeometry {
    fn new(peaks: &WaveData) -> Self {
        Self::Original(Self::draw_waveform(peaks))
    }

    fn rebuild(&mut self, peaks: &WaveData) {
        *self = Self::new(peaks)
    }

    fn get(&self) -> Option<&canvas::Path> {
        match self {
            Self::None => None,
            Self::Scaled { wave, .. } | Self::Original(wave) => Some(wave),
        }
    }

    fn is_interpolated(&self) -> bool {
        matches!(self, Self::Scaled { .. })
    }

    fn interpolate(&mut self, peaks: &WaveData, new_scale: f32) {
        let build_wave = || Self::Scaled {
            scale: new_scale,
            wave: Self::draw_waveform(&wave::interpolate_zoom(peaks, new_scale)),
        };

        match self {
            Self::None | Self::Original(..) => *self = build_wave(),
            Self::Scaled { scale, .. } if *scale != new_scale => *self = build_wave(),
            _ => (),
        }
    }

    fn restore(&mut self, peaks: &WaveData) {
        match self {
            WaveGeometry::Original(_) => (),
            _ => *self = Self::new(peaks),
        }
    }

    fn clear(&mut self) {
        *self = Self::None
    }

    fn draw_waveform(peaks: &WaveData) -> canvas::Path {
        let peaks = &peaks.peaks()[0];

        let mut path = canvas::path::Builder::new();

        // Draw top half of waveform.
        peaks.iter().enumerate().for_each(|(i, local)| {
            path.line_to(Point {
                x: i as f32,
                y: 0.5 - ((1.0 / 2.0) * local.maxima),
            })
        });

        // Draw bottom half of waveform.
        // Backtrack since we're continuing from the end of the top half.
        peaks.iter().enumerate().rev().for_each(|(i, local)| {
            path.line_to(Point {
                x: i as f32,
                y: 0.5 - ((1.0 / 2.0) * local.minima),
            })
        });

        path.close();
        path.build()
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
    wave_id: u64,
    canvas_cache: canvas::Cache,
    waveform: WaveGeometry,
    wave_color: RefCell<Color>,
}

impl State {
    fn new() -> Self {
        Self {
            zoom: 1.0,
            ..Default::default()
        }
    }

    fn reset(&mut self) {
        self.canvas_cache.clear();
        self.waveform.clear();
        self.wave_id = 0;
        self.reset_zoom();
        self.reset_offset();
    }

    fn reset_offset(&mut self) {
        self.previous_offset = 0;
        self.wave_offset = 0;
    }

    fn reset_zoom(&mut self) {
        self.zoom = 1.0;
    }
    
    fn update_zoom(&mut self, wave: &WaveData) {
        self.zoom = self.zoom.clamp(MIN_SCALE, MAX_SCALE);

        match self.zoom < 1.0 {
            true => self.waveform.interpolate(wave, self.zoom),
            false => self.waveform.restore(wave),
        }

        self.canvas_cache.clear();
    }

    fn zoom_in(&mut self, factor: f32, wave: &WaveData) {
        self.zoom *= factor;
        self.update_zoom(wave);
    }

    fn zoom_out(&mut self, factor: f32, wave: &WaveData) {
        self.zoom /= factor;
        self.update_zoom(wave);
    }

    fn update_wave(&mut self, wave: &WaveData) {
        self.waveform.rebuild(wave);
        self.wave_id = wave.id();
        self.update_zoom(wave);
    }

    // Clear canvas cache if wave colors differ
    fn update_wave_color(&self, appearance: &Appearance) {
        use std::ops::DerefMut;

        let new_color = appearance.wave_color;

        match self.wave_color.borrow_mut().deref_mut() {
            old_color if new_color.ne(old_color) => {
                *old_color = new_color;
                self.canvas_cache.clear();
            }
            _ => {}
        }
    }
}

impl<'a, Message, Theme> Widget<Message, Renderer<Theme>> for WaveformViewer<'a, Message, Theme>
where
    Theme: StyleSheet,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn size(&self) -> Size<Length> { 
        Size::new(self.width, self.height)
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer<Theme>,
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
            state.reset();
            return;
        };

        if state.wave_id != wave.id() {
            state.update_wave(wave);
            state.reset_offset();
        }
    }

    fn on_event(
        &mut self,
        _state: &mut widget::Tree,
        event: iced::Event,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer<Theme>,
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

                            let wave = &wave.peaks()[0];

                            let wave_len = (wave.len() as f32 * state.zoom) as usize;

                            state.wave_offset = wave_len.saturating_sub(1).min(new_offset);
                            state.canvas_cache.clear();
                        }
                    }

                    iced::event::Status::Captured
                }

                iced::mouse::Event::WheelScrolled { delta } if cursor_in_bounds() => {
                    let mut zoom_wave = |y: f32| match self.wave {
                        Some(wave) => {
                            match y > 0.0 {
                                true => state.zoom_in(SCALE, wave),
                                false => state.zoom_out(SCALE, wave),
                            };
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
                        state.zoom_in(SCALE, wave);
                        iced::event::Status::Captured
                    }
                    None => iced::event::Status::Ignored,
                },
                KeyCode::Down => match self.wave {
                    Some(wave) => {
                        state.zoom_out(SCALE, wave);
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
        renderer: &mut Renderer<Theme>,
        theme: &Theme,
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

        // Helper function to render marker lines on the waveform
        fn draw_line<Theme>(
            renderer: &mut Renderer<Theme>,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
            background: Color,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle { x, y, width, height },
                    border_radius: 0.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                background,
            );
        }

        let dc_offset = Point {
            x: layout.bounds().x,
            y: layout.bounds().center_y(),
        };

        let layout_width = layout.bounds().width;
        let layout_height = layout.bounds().height;

        // Draw DC line
        let line_thickness = 1.5;
        draw_line(
            renderer,
            layout.bounds().x,
            dc_offset.y - (line_thickness / 2.0),
            layout_width,
            line_thickness,
            appearance.wave_color,
        );

        let state = tree.state.downcast_ref::<State>();

        // Draw waveform
        if let Some((peaks, waveform)) = self
            .wave
            .and_then(|wavedata| Some((wavedata, state.waveform.get()?)))
        {
            state.update_wave_color(&appearance);

            let waveform = state
                .canvas_cache
                .draw(renderer, layout.bounds().size(), |frame| {
                    // Interpolated waveform represents a scaled version of the waveform,
                    // so we don't stretch the frame.
                    let frame_stretch = match state.waveform.is_interpolated() {
                        true => 1.0,
                        false => state.zoom,
                    };

                    // Scale and stretch waveform
                    frame.scale_nonuniform([frame_stretch, layout_height]);

                    // Horizontally transform waveform.
                    frame.translate(Vector {
                        x: 0.0 - (state.wave_offset as f32 / frame_stretch),
                        y: 0.0,
                    });

                    // Color the waveform
                    frame.fill(waveform, appearance.wave_color);
                });

            renderer.with_translation(Vector::new(layout.bounds().x, layout.bounds().y), |renderer| {
                renderer.draw(vec![waveform]);
            });

            // Draw markers - only do so if we're rendering the waveform
            if let Some(markers) = &self.markers {
                let wave_width = peaks.peaks()[0].len() as f32 * state.zoom;

                for marker in markers {
                    let x = layout.bounds().x + wave_width * marker.0 - state.wave_offset as f32;

                    if !layout.bounds().contains([x, dc_offset.y].into()) {
                        continue;
                    }

                    renderer.with_layer(layout.bounds(), |renderer| {
                        draw_line(
                            renderer,
                            x,
                            layout.bounds().y,
                            2.0,
                            layout.bounds().height,
                            appearance.cursor_color,
                        );
                    })
                }
            }
        }

        // Draw cursor
        if cursor.is_over(layout.bounds()) {
            if let Some(Point { x, .. }) = cursor.position() {
                renderer.with_layer(layout.bounds(), |renderer| {
                    draw_line(
                        renderer,
                        x,
                        layout.bounds().y,
                        2.0,
                        layout.bounds().height,
                        match state.mouse_down {
                            true => appearance.wave_color,
                            false => appearance.cursor_color,
                        },
                    )
                });
            }
        }
    }
}

impl<'a, Message, Theme> From<WaveformViewer<'a, Message, Theme>> for Element<'a, Message, Renderer<Theme>>
where
    Message: 'a,
    Theme: StyleSheet + 'a,
{
    fn from(wave_form: WaveformViewer<'a, Message, Theme>) -> Self {
        Self::new(wave_form)
    }
}
