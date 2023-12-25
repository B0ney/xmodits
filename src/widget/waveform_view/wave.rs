use std::sync::atomic::{AtomicU64, Ordering};

static ID: AtomicU64 = AtomicU64::new(1);

fn unique_id() -> u64 {
    ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone)]
pub struct WaveData {
    id: u64,
    peaks: Vec<Vec<Local>>,
}

impl WaveData {
    pub fn new<L>(wave: Vec<Vec<L>>) -> Self
    where
        Local: From<L>,
    {
        Self::from(wave)
    }

    pub fn peaks(&self) -> &Vec<Vec<Local>> {
        &self.peaks
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

impl<L> From<Vec<Vec<L>>> for WaveData
where
    Local: From<L>,
{
    fn from(value: Vec<Vec<L>>) -> Self {
        Self {
            id: unique_id(),
            peaks: value
                .into_iter()
                .map(|f| f.into_iter().map(Local::from).collect())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Local {
    pub maxima: f32,
    pub minima: f32,
}

impl Local {
    pub fn check(mut self) -> Self {
        if self.minima > self.maxima {
            std::mem::swap(&mut self.maxima, &mut self.minima);
        }

        self
    }
}

impl From<(f32, f32)> for Local {
    fn from(value: (f32, f32)) -> Self {
        Self {
            maxima: value.0,
            minima: value.1,
        }
        .check()
    }
}

impl From<Local> for [f32; 2] {
    fn from(value: Local) -> Self {
        let Local { maxima, minima } = value.check();
        [maxima, minima]
    }
}

impl From<[f32; 2]> for Local {
    fn from(value: [f32; 2]) -> Self {
        Self {
            maxima: value[0],
            minima: value[1],
        }
        .check()
    }
}

impl From<f32> for Local {
    fn from(value: f32) -> Self {
        Self {
            maxima: value,
            minima: value,
        }
    }
}

// use linear interpolation to regenerate wave peaks at the desired scale
pub fn interpolate_zoom(wave: &WaveData, factor: f32) -> WaveData {
    use dasp::interpolate::linear::Linear;
    use dasp::signal::{self, Signal};
    use signal::interpolate::Converter;

    let locals: Vec<Vec<Local>> = wave
        .peaks()
        .iter()
        .map(|wave| {
            let mut output = Vec::new();
            let first = wave[0].into();
            let second = wave.get(1).cloned().unwrap_or_default().into();

            let signal = wave.iter().map(|f| [f.maxima, f.minima]);
            let mut converter = Converter::scale_playback_hz(
                signal::from_iter(signal),
                Linear::new(first, second),
                1.0 / factor as f64,
            );

            while !converter.is_exhausted() {
                output.push(Local::from(converter.next()))
            }

            output
        })
        .collect();

    WaveData::from(locals)
}
