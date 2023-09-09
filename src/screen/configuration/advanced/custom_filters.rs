use std::{num::NonZeroU32, path::Path};

use crate::utils::{extension, filename};

/// TODO
pub struct Regex(String);

#[derive(Debug, Default, Clone)]
pub struct PathFilter {
    contains: Vec<String>,
    starts_with: Vec<String>,
    ends_with: Vec<String>,
    has_extension: Vec<String>,
    // has_root: Items,
    before: Option<chrono::DateTime<chrono::Utc>>,
    after: Option<chrono::DateTime<chrono::Utc>>,
    min_size: Option<NonZeroU32>,
    max_size: Option<NonZeroU32>,
}

impl PathFilter {
    fn a(&self, path: &Path) {
        let filename = filename(path);
        let extension = extension(path);
        let meta = path.metadata().ok().unwrap();

        let result = self.contains.has(filename)
            && self.starts_with.has(filename)
            && self.ends_with.has(filename)
            && self.has_extension.has(extension);
        // && self.
    }
}

pub trait HasItem {
    type Item<'a>;
    fn has(&self, item: Self::Item<'_>) -> bool;
}

impl HasItem for Vec<String> {
    type Item<'a> = &'a str;

    fn has(&self, item: &str) -> bool {
        if self.is_empty() {
            return true;
        }
        contains(self, item)
    }
}

pub struct File {}

fn contains(items: &Vec<String>, item: &str) -> bool {
    items.iter().any(|f| f == item)
}

// fn
