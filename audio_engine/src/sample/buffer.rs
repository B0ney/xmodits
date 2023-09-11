use data::xmodits_lib::dsp;


#[derive(Clone)]
pub struct SampleBuffer {
    pub buf: Vec<Vec<f32>>,
    pub rate: u32,
}

impl SampleBuffer {
    pub fn new(buf: Vec<Vec<f32>>, rate: u32) -> Self {
        Self { buf, rate }
    }

    pub fn audio(&self) -> &[Vec<f32>] {
        &self.buf
    }

    pub fn channels(&self) -> usize {
        self.buf.len()
    }

    pub fn frames(&self) -> usize {
        let Some(buffer) = self.buf.get(0) else {
            return 0;
        };
        buffer.len()
    }
}

// impl From<dsp::SampleBuffer> for SampleBuffer {
//     fn from(value: dsp::SampleBuffer) -> Self {
//         todo!()
//     }
// }