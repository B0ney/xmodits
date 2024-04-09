use std::{fmt::Debug, time::Duration};

#[derive(Clone)]
pub struct SampleBuffer {
    pub buf: Vec<Vec<f32>>,
    pub rate: u32,
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
        let Some(buffer) = self.buf.first() else {
            return 0;
        };
        buffer.len()
    }

    pub fn duration(&self) -> std::time::Duration {
        Duration::from_nanos(((self.frames() as f64 / self.rate as f64) * 1_000_000_000.0) as u64)
    }

    pub fn get_sample(&self, frame: usize) -> Option<f32> {
        self.buf[frame % self.channels()]
            .get(frame / self.channels())
            .copied()
    }

    pub fn peaks(&self, interval: Duration) -> Vec<Vec<(f32, f32)>> {
        self.buf
            .iter()
            .map(|channel| peak(channel, self.rate, interval))
            .collect()
    }
}

// calculate peaks and troughs from audio channel.
fn peak(buf: &[f32], rate: u32, interval: Duration) -> Vec<(f32, f32)> {
    let chunks = ((rate as f64 / 1000.0) * (interval.as_millis() as f64)).round() as usize;
    let chunks = chunks.max(1);
    
    let min_max = |x: &[f32]| -> (f32, f32) {
        let mut max = -1.0_f32;
        let mut min = 1.0_f32;

        for i in x.iter().copied() {
            max = max.max(i);
            min = min.min(i);
        }

        (max, min)
    };

    buf.chunks(chunks).map(min_max).collect()
}

impl From<xmodits_lib::dsp::SampleBuffer> for SampleBuffer {
    fn from(sb: xmodits_lib::dsp::SampleBuffer) -> Self {
        let rate = sb.rate_original().clamp(1, u32::MAX);
        Self::new(sb.buf, rate)
    }
}

impl Debug for SampleBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SampleBuffer")
            .field("duration", &self.duration())
            .field("rate", &self.rate)
            .finish()
    }
}
