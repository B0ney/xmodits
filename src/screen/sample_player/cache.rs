use std::{path::PathBuf, collections::HashMap};

use audio_engine::SamplePack;
use parking_lot::RwLock;

pub struct Cache {
    cache: RwLock<HashMap<PathBuf, SamplePack>>,
}