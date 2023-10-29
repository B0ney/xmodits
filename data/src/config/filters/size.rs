use std::path::Path;

use serde::{Deserialize, Serialize};

use super::Filter;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Size {
    pub min: u64,
    pub max: u64,
}

impl Filter for Size {
    fn matches(&self, path: &Path) -> bool {
        let Ok(metadata) = path.metadata() else {
            return false;
        };

        if self.max == 0 {
            self.min >= metadata.len()
        } else {
            (self.min..=self.max).contains(&metadata.len())
        }
    }
}
