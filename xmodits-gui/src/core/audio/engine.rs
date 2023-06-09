use super::core::{AudioOutputDevice, Frame, PlayHandle};
use super::device;
use ringbuf;

const BUFFER_SIZE_LATENCY: usize = 256;

pub struct AudioEngine {
    device: Box<dyn AudioOutputDevice>,
    pub handles: Vec<Box<dyn PlayHandle>>,
    state: State,
    // ring_buffer: ringbuf::HeapRb<Frame>,
    // processing_buffer: Vec<Frame>,
}

impl AudioEngine {
    pub fn init() -> Self {
        let device = Box::new(device::cpal::OutputDevice::init());
        let buffer_size = device.rate() as usize * (BUFFER_SIZE_LATENCY / 1000);
        // let frames_per_tick = ;
        Self {
            device,
            handles: Vec::with_capacity(128),
            state: State::Stop,
            // ring_buffer: ringbuf::HeapRb::new(buffer_size),
        }
    }
    pub fn start() {}
    pub fn tick(&mut self) {
        let mut frame: Frame = [0.0, 0.0];
        let mut to_remove: Vec<usize> = Vec::new();

        for (index, handle) in self.handles.iter_mut().enumerate() {
            let Some(next_frame) = handle.next() else {
                to_remove.push(index);
                continue;
            };

            frame[0] += next_frame[0];
            frame[1] += next_frame[1];
        }

        while let Some(i) = to_remove.pop() {
            self.handles.remove(i);
        }

        self.device.write(&[frame]);
    }
}

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Stop,
    Playing,
    Pause,
}

pub struct Oscillator {
    pub sample_rate: f32,
    pub frame: u64,
    pub frequency: f32,
    pub duration: usize,
}

impl Oscillator {
    fn next_sample(&mut self) {
        self.frame = (self.frame + 1) % self.sample_rate as u64;
    }

    fn sine(&mut self) -> f32 {
        self.next_sample();
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.frame as f32 * self.frequency * two_pi / self.sample_rate).sin()
    }
}

impl PlayHandle for Oscillator {
    fn next(&mut self) -> Option<[f32; 2]> {
        let s = self.sine();
        Some([s, s])
    }

    fn reset(&mut self) {}

    fn jump(&mut self, _tick: u64) {}
}

pub struct Siren {
    sample_rate: f32,
    high: f32,
    low: f32,
    frame: u64,
    rate: f32,
    switch: bool,
}

impl Siren {
    fn next_sample(&mut self) {
        self.frame = (self.frame + 1) % self.sample_rate as u64;
    }
    fn sine(&self, freq: f32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.frame as f32 * freq * two_pi / self.sample_rate).sin()
    }

    fn next(&mut self) -> f32 {
        self.next_sample();
        if self.frame % (self.sample_rate / self.rate).floor() as u64 == 0 {
            self.switch = !self.switch;
        }

        match self.switch {
            true => self.sine(self.high),
            false => self.sine(self.low),
        }
    }
}
impl PlayHandle for Siren {
    fn next(&mut self) -> Option<[f32; 2]> {
        // if self.duration > 44100 {
        //     return None;
        // }
        let s = self.next();
        // self.duration += 1;
        // dbg!(s);
        Some([s, s])
    }

    fn reset(&mut self) {}

    fn jump(&mut self, _tick: u64) {}
}

#[test]
fn a() {
    let mut engine = AudioEngine::init();
    // engine.handles.push(Box::new(Oscillator {
    //     sample_rate: engine.device.rate() as f32,
    //     frame: 0,
    //     frequency: 440.0,
    //     duration: 0,
    // }));
    // engine.handles.push(Box::new(Siren {
    //     sample_rate: engine.device.rate() as f32,
    //     frame: 0,
    //     high: 500.0,
    //     low: 300.0,
    //     rate: 10.0,
    //     switch: true,
    // }));

    engine.handles.push(Box::new(Siren {
        sample_rate: engine.device.rate() as f32,
        frame: 0,
        high: 700.0,
        low: 400.0,
        rate: 5.0,
        switch: true,
    }));
    engine.handles.push(Box::new(Siren {
        sample_rate: engine.device.rate() as f32,
        frame: 4410,
        high: 700.0,
        low: 400.0,
        rate: 5.0,
        switch: true,
    }));
    // engine.handles.push(Box::new(Siren {
    //     sample_rate: engine.device.rate() as f32,
    //     frame: engine.device.rate() as u64 / 4,
    //     high: 700.0,
    //     low: 400.0,
    //     rate: 1.0,
    //     switch: true,
    // }));
    // engine.handles.push(Box::new(Siren {
    //     sample_rate: engine.device.rate() as f32,
    //     frame: 0,
    //     high: 660.0,
    //     low: 220.0,
    //     rate: 2.0,
    // }));

    // engine.handles.push(Box::new(Oscillator {
    //     sample_rate: 44800.0,
    //     frame: 0,
    //     frequency: 880.0,
    //     duration: 0,
    // }));

    // std::thread::spawn(move || {
    loop {
        engine.tick();
        // std::thread::sleep(std::time::Duration::from_nanos(3));
    }
    // }
    // );
}
