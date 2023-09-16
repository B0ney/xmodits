//! When we need to stop the ripping process for some reason.
//! 
//! Uses atomics internally since the extraction uses threads to parallelize
//! the process. 
//! 
//! Additionally, if one of the threads panic for whatever reason, the crash handler will kick in.
//! 
//! The crash handler MUST be able to abort the ripping process, otherwise we could have multiple panics
//! (and multiple error boxes).

use std::sync::atomic::{AtomicU8, Ordering};

static STOP_FLAG: AtomicU8 = AtomicU8::new(0);

const NONE: u8 = 0;
const CANCEL: u8 = 1;
const ABORT: u8 = 2;

/// Has the stopped flag been set?
pub fn is_set() -> bool {
    STOP_FLAG.load(Ordering::Relaxed) != NONE
}

/// Has the cancelled flag been set?
pub fn is_cancelled() -> bool {
    STOP_FLAG.load(Ordering::Relaxed) == CANCEL
}

/// Has the aborted flag been set?
pub fn is_aborted() -> bool {
    STOP_FLAG.load(Ordering::Relaxed) == ABORT
}

/// Reset the flag back to its original state
pub fn reset() {
    STOP_FLAG.store(NONE, Ordering::Relaxed);
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
#[repr(u8)]
pub enum StopFlag {
    None = NONE,
    Cancel = CANCEL,
    Abort = ABORT,
}

pub fn set_flag(flag: StopFlag) {
    STOP_FLAG.store(flag as u8, Ordering::Relaxed);
}

pub fn get_flag() -> StopFlag {
    match STOP_FLAG.load(Ordering::Relaxed) {
        NONE => StopFlag::None,
        CANCEL => StopFlag::Cancel,
        ABORT => StopFlag::Abort,
        _ => unreachable!()
    }
}
