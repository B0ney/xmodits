use std::path::PathBuf;

pub struct Error {
    file: PathBuf,
    reason: String,
}

struct History {
    entries: Vec<PathBuf>,
    errors: Option<Vec<Error>>
}

struct Task {
    
}