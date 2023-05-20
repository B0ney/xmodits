use std::hash::{Hash, Hasher};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

// use chrono::Utc;

use super::cfg::SampleRippingConfig;

#[derive(Default)]
pub struct Entry {
    pub selected: bool,
    pub path: PathBuf,
    filename: Box<str>,
}

impl Entry {
    pub fn new(path: PathBuf) -> Self {
        let filename = path
            .file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or_default()
            .into();

        Self {
            selected: false,
            path: path.into(),
            filename,
        }
    }
    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }
}

impl Hash for Entry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.selected.hash(state);
        self.path.hash(state);
    }
}

#[derive(Default)]
pub struct Entries {
    pub all_selected: bool,
    pub entries: Vec<Entry>, // todo: use hashset
}

impl Entries {
    pub fn contains(&self, path: &Path) -> bool {
        self.entries.iter().any(|x| &*x.path == path)
    }

    pub fn add(&mut self, path: PathBuf) {
        self.entries.push(Entry::new(path));
    }

    pub fn total_selected(&self) -> usize {
        self.entries.iter().filter(|f| f.selected).count()
    }

    pub fn clear(&mut self) {
        self.all_selected = false;
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn select(&mut self, index: usize, selected: bool) {
        if let Some(entry) = self.entries.get_mut(index) {
            entry.selected = selected;
        }
        self.all_selected = self.total_selected() == self.entries.len();
    }

    pub fn select_all(&mut self, selected: bool) {
        if selected && self.entries.is_empty() {
            return;
        }
        self.all_selected = selected;
        self.entries
            .iter_mut()
            .for_each(|entry| entry.selected = selected);
    }

    pub fn take_selected(&mut self) -> Vec<Entry> {
        if self.all_selected {
            return std::mem::take(&mut self.entries);
        }

        let mut selected = Vec::with_capacity(self.total_selected());
        let entries = &mut self.entries;

        let mut i = 0;

        while i < entries.len() {
            if entries[i].selected {
                selected.push(entries.remove(i));
            } else {
                i += 1;
            }
        }
        selected
    }

    pub fn files(&self) -> usize {
        self.entries.iter().filter(|f| f.is_file()).count()
    }
}

#[derive(Default)]
pub struct History {
    // timestamp: chrono::DateTime<Utc>,
    entries: Entries,
    failed: Option<Failed>,
    config: SampleRippingConfig,
}

enum Failed {
    Mem(Vec<FailedModule>),
    File { path: PathBuf, file: File },
}

struct FailedModule {
    path: PathBuf,
    reason: String,
}
