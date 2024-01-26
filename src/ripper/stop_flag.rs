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

/// Reset the flag back to its original state (only if abort flag isn't set)
pub fn reset() {
    if !is_aborted() {
        STOP_FLAG.store(NONE, Ordering::Relaxed);
    }
}

/// TODO: This should only be called by the panic handler.
/// But I have no idea how to enforce this... 
/// 
/// `pub(in crate::logger::crash_handler)` doesn't work
/// 
/// For now, we'll just use `track_caller` to keep a close eye on it...
#[track_caller]
pub(in crate) fn set_abort() {
    STOP_FLAG.store(ABORT, Ordering::Relaxed);
    tracing::warn!("ABORT triggered from: {}", std::panic::Location::caller());
}

/// Set flag to cancel (only if abort flag isn't set)
pub fn set_cancel() {
    if !is_aborted() {
        STOP_FLAG.store(CANCEL, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
#[repr(u8)]
pub enum StopFlag {
    None = NONE,
    Cancel = CANCEL,
    Abort = ABORT,
}

pub fn get_flag() -> StopFlag {
    match STOP_FLAG.load(Ordering::Relaxed) {
        NONE => StopFlag::None,
        CANCEL => StopFlag::Cancel,
        ABORT => StopFlag::Abort,
        _ => unreachable!(),
    }
}
