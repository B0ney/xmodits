use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use crate::utils::filename;

#[derive(Default)]
pub struct Entry {
    pub selected: bool,
    pub path: PathBuf,
}

impl Entry {
    pub fn new(path: PathBuf) -> Self {
        Self {
            selected: false,
            path: path,
        }
    }
    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.path.is_file()
    }

    pub fn filename(&self) -> &str {
        filename(&self.path)
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
    pub entries: Vec<Entry>, // todo: use hashset, use "(bool, Entry)" instead?
}

impl Entries {
    pub fn contains(&self, path: &Path) -> bool {
        self.entries.iter().any(|x| &*x.path == path)
    }

    pub fn add(&mut self, path: PathBuf) {
        if self.contains(&path) {
            return;
        }

        self.entries.push(Entry::new(path));
        self.all_selected = false;
    }

    pub fn all_selected(&self) -> bool {
        self.all_selected
    }

    /// todo: avoid duplicates.
    pub fn add_multiple(&mut self, paths: Vec<PathBuf>) {
        self.all_selected = false;
        paths
            .into_iter()
            .for_each(|path| self.entries.push(Entry::new(path)))
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

    pub fn delete_selected(&mut self) {
        // clear the entries if everything is selected
        if self.all_selected || self.total_selected() == self.entries.len() {
            self.entries.clear();
            // self.current = None;
            return;
        }

        let mut i = 0;

        while i < self.entries.len() {
            let path = &self.entries[i];
            if path.selected {
                // if matches!(&self.current, Some(e) if e.matches(&path.path)) {
                //     self.current = None;
                // }
                let _ = self.entries.remove(i);
            } else {
                i += 1;
            }
        }
        self.all_selected = false;
    }

    pub fn files(&self) -> usize {
        self.entries.iter().filter(|f| f.is_file()).count()
    }

    pub fn invert(&mut self) {
        if self.is_empty() {
            return;
        }

        if self.all_selected || self.non_selected() {
            self.all_selected = !self.all_selected;
        };

        self.entries
            .iter_mut()
            .for_each(|f| f.selected = !f.selected)
    }

    pub fn non_selected(&self) -> bool {
        self.total_selected() == 0
    }

    pub fn get(&self, idx: usize) -> Option<&Path> {
        self.entries.get(idx).map(|f| f.path.as_ref())
    }
}

fn view(entry: &Entry) {}
