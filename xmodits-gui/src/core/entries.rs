use std::{fs::File, path::{PathBuf, Path}};
use std::hash::{Hash, Hasher};

use chrono::Utc;
use xmodits_lib::interface::Error;

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

    pub fn select(&mut self, index: usize, selected: bool) {
        if let Some(entry) = self.entries.get_mut(index) {
            entry.selected = selected;
        }
    }
}


#[derive(Default)]
pub struct History {
    timestamp: chrono::DateTime<Utc>,
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
