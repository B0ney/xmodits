use std::sync::atomic::{AtomicU8, Ordering};

/// When we need to stop the ripping process for some reason.
static STOP_FLAG: AtomicU8 = AtomicU8::new(0);

const NONE: u8 = 0;
const CANCEL: u8 = 1;
const ABORT: u8 = 2;

/// Has the stopped flag been set?
pub fn stopped() -> bool {
    STOP_FLAG.load(Ordering::Relaxed) != NONE
}

/// Has the cancelled flag been set?
pub fn cancelled() -> bool {
    STOP_FLAG.load(Ordering::Relaxed) == CANCEL
}

/// Has the aborted flag been set?
pub fn aborted() -> bool {
    STOP_FLAG.load(Ordering::Relaxed) == ABORT
}

/// Reset the flag back to its original state
pub fn reset() {
    STOP_FLAG.store(NONE as u8, Ordering::Relaxed);
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum StopFlag {
    None = NONE,
    Cancel = CANCEL,
    Abort = ABORT,
}

pub fn set_flag(flag: StopFlag) {
    STOP_FLAG.store(flag as u8, Ordering::Relaxed);
}

