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

#[derive(Debug, Clone, Copy)]
pub struct Local {
    pub maxima: f32,
    pub minima: f32,
}

impl Local {
    pub fn check(mut self) -> Self {
        if self.minima > self.maxima {
            let temp = self.maxima;
            self.maxima = self.minima;
            self.minima = temp;
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
