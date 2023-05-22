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
        self.batch_size.atomic_set(size);
    }

    pub fn set_sub_batch_size(&self, size: usize) {
        self.sub_batch_size.atomic_set(size);
    }

    pub fn incr_batch_number(&self) {
        self.batch_number.atomic_incr();
    }

    pub fn incr_sub_batch_number(&self) {
        self.sub_batch_number.atomic_incr();
    }

    pub fn incr_file(&self) {
        self.sq_idx.atomic_incr();
    }

    pub fn current_path(&self) -> Option<PathBuf> {
        let index = self.sq_idx.atomic_get();
        self.files.lock().get(index).cloned()
    }

    pub fn batch_number(&self) -> u64 {
        self.batch_number.atomic_get()
    }

    pub fn batch_size(&self) -> usize {
        self.batch_size.atomic_get()
    }

    pub fn sub_batch_number(&self) -> u64 {
        self.sub_batch_number.atomic_get()
    }

    pub fn sub_batch_size(&self) -> usize {
        self.sub_batch_size.atomic_get()
    }

    pub fn reset(&self) {
        self.set_batch_size(0);
        self.set_sub_batch_size(0);
        self.sq_idx.atomic_reset();
        self.batch_number.atomic_reset();
        self.sub_batch_number.atomic_reset();
    }
}

pub trait AtomicVariable {
    type Val;

    fn atomic_get(&self) -> Self::Val;
    fn atomic_incr(&self);
    fn atomic_set(&self, size: Self::Val);
    fn atomic_reset(&self);
}

macro_rules! atomic_var_impl(
    ($atomic:ty, $value:ty) => {impl AtomicVariable for $atomic {
        type Val = $value;

        fn atomic_incr(&self) {
            self.fetch_add(1, Ordering::Relaxed);
        }

        fn atomic_set(&self, size: Self::Val) {
            self.store(size, Ordering::Relaxed);
        }

        fn atomic_reset(&self) {
            self.store(0, Ordering::Relaxed);
        }

        fn atomic_get(&self) -> Self::Val {
            self.load(Ordering::Relaxed)
        }
    }}
);

atomic_var_impl!(AtomicUsize, usize);
atomic_var_impl!(AtomicU64, u64);
