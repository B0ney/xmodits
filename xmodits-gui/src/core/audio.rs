use std::collections::HashMap;
use std::sync::{Arc, Weak};
use parking_lot::RwLock;

pub trait AudioOutputDevice {

}

pub struct CpalOutputDevice {

}

pub struct SampleInfo {
    name: String,
    filename: Option<String>,
    buffer: Arc<SampleBuffer>,
}

pub struct AudioDevice {
    out: Box<dyn AudioOutputDevice>,
    cache: RwLock<HashMap<String, Weak<SampleBuffer>>>,
}

pub struct SampleList {
    
}