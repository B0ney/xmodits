use std::sync::Arc;
use parking_lot::Mutex;

pub type Batch<T> = Arc<Mutex<Vec<T>>>;

#[derive(Default, Clone, Copy)]
enum CurrentBatch {
    #[default]
    Batch1,
    Batch2,
}

impl CurrentBatch {
    pub fn next(self) -> Self {
        match self {
            Self::Batch1 => Self::Batch2,
            Self::Batch2 => Self::Batch1,
        }
    }

    pub fn switch(&mut self) {
        *self = self.next();
    }
}

/// Double buffer
pub struct Buffer<T> {
    current: CurrentBatch,
    buf_1: Batch<T>,
    buf_2: Batch<T>,
}

impl<T> Buffer<T> {
    pub fn init(batch_size: usize) -> Self {
        Self {
            current: CurrentBatch::Batch1,
            buf_1: Self::alloc(batch_size),
            buf_2: Self::alloc(batch_size),
        }
    }

    pub fn current_buffer(&self) -> Batch<T> {
        match self.current {
            CurrentBatch::Batch1 => self.buf_1.clone(),
            CurrentBatch::Batch2 => self.buf_2.clone(),
        }
    }

    pub fn switch(&mut self) {
        self.current.switch()
    }

    fn alloc(batch_size: usize) -> Batch<T> {
        Arc::new(Mutex::new(Vec::with_capacity(batch_size)))
    }
}
