use std::path::Path;

use super::Filter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Name {
    pub contains: Vec<String>,
    pub starts_with: Vec<String>,
    pub ends_with: Vec<String>,
    pub extensions: Vec<String>,
}

impl Default for Name {
    fn default() -> Self {
        Self {
            contains: (0..30).into_iter().map(|f| format!("test{f}")).collect(),
            starts_with: ["test", "test2", "test3"].into_iter().map(String::from).collect(),
            ends_with: (0..50).into_iter().map(|f| f.to_string()).collect(),
            extensions: (0..10).into_iter().map(|f| f.to_string()).collect(),
        }
    }
}

impl Filter for Name {
    fn matches(&self, path: &Path) -> bool {
        let filename = filename(path);
        let extension = extension(path);

        contains(&self.contains, |e| e.contains(filename))
            && contains(&self.starts_with, |e| e.starts_with(filename))
            && contains(&self.ends_with, |e| e.ends_with(filename))
            && contains(&self.extensions, |e| e == extension)
    }
}

/// Returns true if items contains an element that satisfies the given predicate OR
/// the items is empty.
fn contains(items: &[String], predicate: impl Fn(&String) -> bool) -> bool {
    items.is_empty() || items.iter().any(predicate)
}

fn filename(path: &Path) -> &str {
    path.file_name().and_then(|f| f.to_str()).unwrap_or_default()
}

fn extension(path: &Path) -> &str {
    path.extension().and_then(|f| f.to_str()).unwrap_or_default()
}
