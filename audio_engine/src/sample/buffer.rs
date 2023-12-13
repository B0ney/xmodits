use std::time::Duration;

#[derive(Clone)]
pub struct SampleBuffer {
    buf: Vec<Vec<f32>>,
    rate: u32,
}

impl SampleBuffer {
    pub fn new(buf: Vec<Vec<f32>>, rate: u32) -> Self {
        assert!(rate != 0, "Sample rate cannot be zero");

        Self { buf, rate }
    }

    pub fn channels(&self) -> usize {
        self.buf.len()
    }

    pub fn rate(&self) -> u32 {
        self.rate
    }

    pub fn frames(&self) -> usize {
        let Some(buffer) = self.buf.get(0) else {
            return 0;
        };
        buffer.len()
    }

    pub fn duration(&self) -> std::time::Duration {
        Duration::from_micros(((self.frames() as f64 / self.rate as f64) * 1_000_000.0) as u64)
    }

    pub fn get_sample(&self, frame: usize) -> Option<f32> {
        self.buf[frame % self.channels()]
            .get(frame / self.channels())
            .copied()
    }
}

impl From<xmodits_lib::dsp::SampleBuffer> for SampleBuffer {
    fn from(sb: xmodits_lib::dsp::SampleBuffer) -> Self {
        let rate = sb.rate_original().clamp(1, u32::MAX);
        Self::new(sb.buf, rate)
    }
}
