use std::path::Path;

pub trait CustomFilter: Fn(&Path) -> bool + Send + Sync {}

impl <T: Fn(&Path) -> bool + Send + Sync>CustomFilter for T {}

