use std::collections::HashMap;

pub struct TrackerInfoView {
    current: Option<Info>,
    // cache: HashMap<>
}

pub struct Info {
    module_name: String,
    format: String,
    samples: usize,
    total_sample_size: usize,
    comments: Option<String>,
}

