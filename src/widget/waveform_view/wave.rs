#[derive(Debug, Clone)]
pub struct WaveData(pub Vec<Vec<f32>>);

#[derive(Debug, Clone, Copy)]
pub struct Local {
    pub maxima: f64,
    pub minima: f64,
}

