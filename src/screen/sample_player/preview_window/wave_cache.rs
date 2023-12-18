use std::collections::HashMap;
use std::path::{Path, PathBuf};

use parking_lot::RwLock;

use crate::widget::waveform::WaveData;

#[derive(Debug, Default)]
pub struct WaveCache {
    pub cache: HashMap<usize, WaveData>,
}

