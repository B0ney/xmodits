use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

use once_cell::sync::Lazy;
use parking_lot::Mutex;

pub static GLOBAL_TRACKER: Lazy<GlobalTracker> = Lazy::new(|| GlobalTracker::init());

/// Will be accessed by the panic Handler
pub struct GlobalTracker {
    sq_idx: AtomicUsize,
    pub files: Arc<Mutex<Vec<PathBuf>>>,
    batch_size: AtomicUsize,
    batch_number: AtomicU64,
    sub_batch_size: AtomicUsize,
    sub_batch_number: AtomicU64,
    pub traversed_file: Arc<Mutex<Option<PathBuf>>>,
}

impl GlobalTracker {
    pub fn init() -> Self {
        Self {
            sq_idx: AtomicUsize::new(0),
            files: Arc::new(Mutex::new(Vec::new())),
            batch_size: AtomicUsize::new(0),
            batch_number: AtomicU64::new(0),
            sub_batch_size: AtomicUsize::new(0),
            sub_batch_number: AtomicU64::new(0),
            traversed_file: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_files(&self, paths: Vec<PathBuf>) -> Arc<Mutex<Vec<PathBuf>>> {
        {
            *self.files.lock().as_mut() = paths;
        }

        self.files.clone()
    }

    pub fn set_batch_size(&self, size: usize) {
        self.batch_size.store(size, Ordering::Relaxed);
    }

    pub fn set_sub_batch_size(&self, size: usize) {
        self.sub_batch_size.store(size, Ordering::Relaxed);
    }

    pub fn incr_batch_number(&self) {
        self.batch_number.fetch_add(1, Ordering::Relaxed);
    }

    pub fn incr_sub_batch_number(&self) {
        self.sub_batch_number.fetch_add(1, Ordering::Relaxed);
    }

    pub fn incr_file(&self) {
        self.sq_idx.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_current_path(&self) -> Option<PathBuf> {
        let index = self.sq_idx.load(Ordering::Relaxed);
        self.files.lock().get(index).cloned()
    }

    pub fn get_batch_number(&self) -> u64 {
        self.batch_number.load(Ordering::Relaxed)
    }

    pub fn get_batch_size(&self) -> usize {
        self.batch_size.load(Ordering::Relaxed)
    }

    pub fn get_sub_batch_number(&self) -> u64 {
        self.sub_batch_number.load(Ordering::Relaxed)
    }

    pub fn get_sub_batch_size(&self) -> usize {
        self.sub_batch_size.load(Ordering::Relaxed)
    }

    pub fn reset(&self) {
        self.set_batch_size(0);
        self.set_sub_batch_size(0);
        self.sq_idx.store(0, Ordering::Relaxed);
        self.batch_number.store(0, Ordering::Relaxed);
        self.sub_batch_number.store(0, Ordering::Relaxed);
    }
}
