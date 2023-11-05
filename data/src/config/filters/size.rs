use std::path::Path;

use serde::{Deserialize, Serialize};

use super::Filter;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Size {
    pub min: u64,
    pub max: u64,
    pub min_modifier: Modifier,
    pub max_modifier: Modifier,
}

impl Size {
    fn min_as_bytes(&self) -> u64 {
        self.min * self.min_modifier as u64
    }

    fn max_as_bytes(&self) -> u64 {
        self.max * self.max_modifier as u64
    }
}

impl Filter for Size {
    fn matches(&self, path: &Path) -> bool {
        let Ok(metadata) = path.metadata() else {
            return false;
        };

        if self.max_as_bytes() == 0 {
            self.min_as_bytes() >= metadata.len()
        } else {
            (self.min_as_bytes()..=self.max_as_bytes()).contains(&metadata.len())
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self {
            min: 1,
            max: 40,
            min_modifier: Modifier::B,
            max_modifier: Modifier::MB,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, Eq, PartialEq)]
#[repr(u64)]
pub enum Modifier {
    B = 1,
    #[default]
    KB = 1_000,
    MB = 1_000_000,
}

impl Modifier {
    pub const ALL: &[Self] = &[Self::B, Self::KB, Self::MB];
}

impl std::fmt::Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Modifier::B => "bytes",
                Modifier::KB => "KB",
                Modifier::MB => "MB",
            }
        )
    }
}
