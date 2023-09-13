use tokio::sync::mpsc::Sender;

use super::{Signal, stop_flag};

/// Communicates with the subscription
#[derive(Default)]
pub struct SubscriptionHandle {
    start_signal: Option<Sender<Signal>>
}

impl SubscriptionHandle {
    pub fn new() -> Self {
        Self { start_signal: None }
    }

    pub fn set(&mut self, sender: Sender<Signal>) {
        self.start_signal = Some(sender)
    }
    
    pub fn send(&self, signal: Signal) {
        if let Some(sender) = self.start_signal.as_ref() {
            // TODO: should this return the value on error?
            let _ = sender.send(signal);
        }
    }

    pub fn is_active(&self) -> bool {
        match self.start_signal.as_ref() {
            Some(signal) => !signal.is_closed(),
            None => false,
        }
    }

    pub fn cancel() {
        stop_flag::set_flag(stop_flag::StopFlag::Cancel)
    }

    pub fn cancelled() -> bool {
        stop_flag::cancelled()
    }

    pub fn aborted() -> bool {
        stop_flag::aborted()
    }
}