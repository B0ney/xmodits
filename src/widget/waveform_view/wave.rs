#[derive(Debug, Clone)]
pub struct WaveData(pub Vec<Vec<Local>>);

impl<L> From<Vec<Vec<L>>> for WaveData
where
    Local: From<L>,
{
    fn from(value: Vec<Vec<L>>) -> Self {
        Self(
            value
                .into_iter()
                .map(|f| f.into_iter().map(Local::from).collect())
                .collect(),
        )
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
