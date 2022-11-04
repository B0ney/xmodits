use std::path::{PathBuf, Path};



pub enum Trackers {
    File {
        path: PathBuf,
        filename: String,
    },
    Folder {
        path: PathBuf

    }
}

impl Trackers {
    pub fn view(&self) {
        
    }

    pub fn path(&self) -> &Path {
        match self {
            Trackers::File{ path, .. } => &path,
            Trackers::Folder{ path, .. } => &path,
        }
    }
}