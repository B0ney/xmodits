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
            path,
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

#[derive(Default)]
pub struct Entries {
    pub all_selected: bool,
    pub entries: Vec<Entry>,
}

impl Entries {
    pub fn contains(&self, path: &Path) -> bool {
        self.entries.iter().any(|x| &*x.path == path)
    }

    pub fn all_selected(&self) -> bool {
        self.all_selected
    }

    pub fn add(&mut self, path: PathBuf) {
        if self.contains(&path) {
            return;
        }
        self.entries.push(Entry::new(path));
        self.all_selected = false;
    }

    pub fn add_multiple(&mut self, paths: Vec<PathBuf>) {
        paths.into_iter().for_each(|path| self.add(path));
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

    pub fn take(&mut self) -> Vec<PathBuf> {
        match self.none_selected() {
            true => std::mem::take(&mut self.entries),
            false => self.take_selected(),
        }
        .into_iter()
        .map(|f| f.path)
        .collect()
    }

    pub fn delete_selected(&mut self, current: Option<&Path>) -> bool {
        // clear the entries if everything is selected
        if self.all_selected || self.total_selected() == self.entries.len() {
            self.entries.clear();
            return true;
        }

        self.all_selected = false;

        let mut clear_current_tracker = false;
        
        self.entries.retain(|entry: &Entry| {
            if current.is_some_and(|path| path == entry.path) {
                clear_current_tracker = true;
            }

            !entry.selected
        });

        clear_current_tracker
    }

    pub fn files(&self) -> usize {
        self.entries.iter().filter(|f| f.is_file()).count()
    }

    pub fn invert(&mut self) {
        if self.is_empty() {
            return;
        }

        if self.all_selected || self.none_selected() {
            self.all_selected = !self.all_selected;
        };

        self.entries
            .iter_mut()
            .for_each(|f| f.selected = !f.selected)
    }

    pub fn none_selected(&self) -> bool {
        self.total_selected() == 0
    }

    pub fn get(&self, idx: usize) -> Option<&Path> {
        self.entries.get(idx).map(|f| f.path.as_ref())
    }
}
