pub type Frame = [f32; 2];
use super::engine::AudioEngineHandle;

pub const DEFAULT_RATE: u32 = 44100;
pub const DEFAULT_BUFFER_SIZE: usize = 2048;

pub enum Event {
    RequestAudioDeviceReset,
    PushPlayHandle(Box<dyn PlayHandle>),
    PlayEvent(super::engine::State),
    Clear,
}

pub trait AudioOutputDevice {
    fn init(handle: AudioEngineHandle) -> Box<Self> where Self: Sized;
    fn rate(&self) -> u32;
    fn reset(&mut self);
    fn write(&mut self, chunk: &[[f32; 2]]);
}

pub trait PlayHandle: Send + Sync {
    fn next(&mut self) -> Option<[f32; 2]>;
    fn reset(&mut self);
    fn jump(&mut self, tick: usize);
}

pub trait FrameModifier {
    fn clamp(self) -> Self;
    fn amplify(self, value: f32) -> Self;
}

impl FrameModifier for [f32; 2] {
    fn clamp(self) -> Self {
        self.map(|s| s.clamp(-1.0, 1.0))
    }

    fn amplify(self, value: f32) -> Self {
        self.map(|s| (s * value))
    }
}
