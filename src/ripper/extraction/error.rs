use std::path::PathBuf;

use xmodits_lib::Error;

use crate::utils::filename;


#[derive(Debug, Clone)]
pub struct Failed {
    pub path: PathBuf,
    pub reason: Reason,
}

impl std::fmt::Display for Failed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed: {}, reason: {:?}",
            self.path.display(),
            &self.reason
        )
    }
}

#[derive(Debug, Clone)]
pub enum Reason {
    Single(String),
    Multiple(Vec<(usize, String)>),
}

impl Failed {
    pub fn new(path: String, error: Error) -> Self {
        let path: PathBuf = path.into();
        let reason = match error {
            Error::FailedRip(multi) => Reason::Multiple(
                multi
                    .inner()
                    .into_iter()
                    .map(|reason| (reason.raw_index, reason.reason.to_string()))
                    .collect(),
            ),
            single => Reason::Single(single.to_string()),
        };

        Self { path, reason }
    }

    pub fn filename(&self) -> &str {
        filename(&self.path)
    }
}