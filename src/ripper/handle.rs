use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::Sender;

use super::{stop_flag, Signal};

/// Communicates with the subscription.
///
/// Also provides convenience methods to access the global stop_flag.
#[derive(Default)]
pub struct Handle {
    sender: Option<Sender<Signal>>,
}

impl Handle {
    pub fn new() -> Self {
        Self { sender: None }
    }

    pub fn set_sender(&mut self, sender: Sender<Signal>) {
        tracing::info!("Received sender!");
        self.sender = Some(sender)
    }

    pub fn send(&self, signal: Signal) -> Result<(), Signal> {
        let Some(sender) = self.sender.as_ref() else {
            return Err(signal);
        };

        let get_sender = |err: TrySendError<Signal>| -> Signal {
            match err {
                TrySendError::Full(sender) | TrySendError::Closed(sender) => sender,
            }
        };

        sender.try_send(signal).map_err(get_sender)
    }

    pub fn is_active(&self) -> bool {
        self.sender
            .as_ref()
            .is_some_and(|sender| !sender.is_closed())
    }

    /// Cancel the ripping process by setting the stop_flag to Cancel
    pub fn cancel(&self) {
        stop_flag::set_flag(stop_flag::StopFlag::Cancel)
    }

    pub fn cancelled(&self) -> bool {
        stop_flag::is_cancelled()
    }

    pub fn aborted(&self) -> bool {
        stop_flag::is_aborted()
    }

    pub fn reset_stop_flag(&self) {
        if !stop_flag::is_aborted() {
            stop_flag::reset()
        }
    }
}
