pub type Frame = [f32; 2];

pub const DEFAULT_RATE: u32 = 41000;
pub const DEFAULT_BUFFER_SIZE: usize = 2048;

pub enum Event {
    RequestAudioDeviceReset,
    PushPlayHandle(Box<dyn PlayHandle>),
    PlayEvent(super::engine::State),
}

pub trait AudioOutputDevice {
    fn rate(&self) -> u32;
    fn reset(&mut self);
    fn write(&mut self, chunk: &[[f32; 2]]);
}

pub trait PlayHandle {
    fn next(&mut self) -> Option<[f32; 2]>;
    fn reset(&mut self);
    fn jump(&mut self, tick: u64);
}
