use std::path::{PathBuf, Path};

use xmodits_lib::traits::Module;

#[derive(Debug, Clone)]
pub enum Info {
    Valid {
        path: PathBuf,
        name: String,
        format: String,
        samples: usize,
        total_sample_size: usize,
    },
    Invalid {
        path: PathBuf,
        error: String,
    },
}

impl Info {
    pub fn matches(&self, other: &Path) -> bool {
        matches!(
            self,
            Self::Invalid { path, .. } |
            Self::Valid { path, ..} if path == other
        )
    }
    pub fn path(&self) -> &Path {
        match self {
            Self::Invalid { path, .. } | Self::Valid { path, .. } => path,
        }
    }
    pub fn valid(tracker: Box<dyn Module>, path: PathBuf) -> Self {
        Self::Valid {
            name: tracker.name().to_owned(),
            format: tracker.format().to_owned(),
            samples: tracker.total_samples(),
            path,
            total_sample_size: tracker
                .samples()
                .iter()
                .map(|f| f.length as usize)
                .sum::<usize>()
                / 1024,
        }
    }
    pub fn invalid(error: String, path: PathBuf) -> Self {
        Self::Invalid { error, path }
    }
}