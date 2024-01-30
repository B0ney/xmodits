use std::time::Instant;

use crate::sample::{FramesIter, TrackerSample};

pub struct SamplePlayer {
    inner: Option<RodioEngine>,
}

impl Default for SamplePlayer {
    fn default() -> Self {
        Self {
            inner: RodioEngine::new(),
        }
    }
}

impl SamplePlayer {
    pub fn create_handle(&self) -> PlayerHandle {
        PlayerHandle {
            inner: self.inner.as_ref().and_then(RodioEngine::create_handle),
        }
    }
}

struct RodioEngine {
    _stream: rodio::OutputStream,
    _handle: rodio::OutputStreamHandle,
}

impl RodioEngine {
    fn new() -> Option<Self> {
        rodio::OutputStream::try_default()
            .ok()
            .map(|(_stream, _handle)| Self { _stream, _handle })
    }

    fn create_handle(&self) -> Option<rodio::Sink> {
        rodio::Sink::try_new(&self._handle).ok()
    }
}

pub struct PlayerHandle {
    inner: Option<rodio::Sink>,
}

impl PlayerHandle {
    pub fn play(&self, source: TrackerSample) {
        self.unpause();
        if let Some(sink) = &self.inner {
            sink.append(FramesIter {
                sample: source,
                timer: Instant::now(),
                callback: None,
            });
        }
    }

    pub fn play_with_callback<F>(&self, source: TrackerSample, callback: F)
    where
        F: Fn(&TrackerSample, &mut Instant) + Send + 'static,
    {
        self.unpause();
        if let Some(sink) = &self.inner {
            sink.append(FramesIter {
                sample: source,
                timer: Instant::now(),
                callback: Some(Box::new(callback)),
            });
        }
    }

    pub fn stop(&self) {
        if let Some(sink) = &self.inner {
            sink.stop();
        }
    }

    pub fn pause(&self) {
        if let Some(sink) = &self.inner {
            if !sink.empty() {
                match sink.is_paused() {
                    true => sink.play(),
                    false => sink.pause(),
                }
            }
        }
    }

    pub fn unpause(&self) {
        if let Some(sink) = &self.inner {
            if sink.is_paused() {
                sink.pause();
                sink.play()
            }
        }
    }

    pub fn is_playing(&self) -> bool {
        self.inner
            .as_ref()
            .is_some_and(|sink| !sink.empty() && !sink.is_paused())
    }

    pub fn set_volume(&self, volume: f32) {
        if let Some(sink) = &self.inner {
            sink.set_volume(volume)
        }
    }

    pub fn is_inactive(&self) -> bool {
        self.inner.is_none()
    }

    pub fn is_active(&self) -> bool {
        self.inner.is_some()
    }
}
