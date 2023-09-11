pub mod error_handler;
pub mod extraction;

use data::time::Time;
use error_handler::ErrorHandler;
use iced::{subscription, Subscription};

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc::{self, Receiver, Sender, UnboundedReceiver};

use tracing::{error, info};

use super::Signal;

pub use extraction::{Failed, Message as ThreadMessage};

pub static CANCELLED: AtomicBool = AtomicBool::new(false);

pub fn cancelled() -> bool {
    CANCELLED.load(Ordering::Relaxed)
}

// pub struct Handle {
//     cancel: &'static AtomicBool,

// }

/// State of subscription
#[derive(Default, Debug)]
enum State {
    #[default]
    Init,
    Idle(Receiver<Signal>),
    Ripping {
        ripping_msg: UnboundedReceiver<ThreadMessage>,
        total: u64,
        progress: u64,
        error_handler: ErrorHandler,
        total_errors: u64,
        timer: Time,
    },
}

/// Messages emitted by subscription
#[derive(Clone, Debug)]
pub enum Message {
    Ready(Sender<Signal>),
    Progress { progress: f32, total_errors: u64 },
    Done { state: CompleteState, time: Time },
    Info(Option<String>),
}

#[derive(Default, Debug, Clone)]
pub enum CompleteState {
    Cancelled,
    #[default]
    NoErrors,
    SomeErrors(Vec<Failed>),
    TooMuchErrors {
        log: PathBuf,
        total: u64,
    },
    TooMuchErrorsNoLog {
        reason: String,
        errors: Vec<Failed>,
        discarded: u64,
        manually_saved: bool,
    },
}

impl CompleteState {
    pub fn take(&mut self) -> Option<Vec<Failed>> {
        Some(std::mem::take(self.errors_ref_mut()?))
    }

    pub fn errors_ref_mut(&mut self) -> Option<&mut Vec<Failed>> {
        match self {
            Self::SomeErrors(errors) | Self::TooMuchErrorsNoLog { errors, .. } => Some(errors),
            _ => None,
        }
    }

    // pub fn set_manually_saved(&mut self) {
    //     if let Self::TooMuchErrorsNoLog { manually_saved, .. } = self {
    //         *manually_saved = true;
    //     }
    // }
}

impl From<ErrorHandler> for CompleteState {
    fn from(value: ErrorHandler) -> Self {
        match value {
            ErrorHandler::Mem { errors, .. } => match errors.is_empty() {
                true => Self::NoErrors,
                false => Self::SomeErrors(errors),
            },
            ErrorHandler::File { total, path, .. } => Self::TooMuchErrors { log: path, total },
            ErrorHandler::FailedFile {
                reason,
                errors,
                discarded,
            } => Self::TooMuchErrorsNoLog {
                reason,
                errors,
                discarded,
                manually_saved: false,
            },
        }
    }
}

/// The subscription will emit messages when:
///
/// * It has been (re)initialized. This is so that the app can send the files/folders to rip and the configuration.
/// * The worker sends custom messages to keep the user updated. E.g ``"Traversing folders..."``, ``"Ripping 100 files..."``
/// * A module has/can't be ripped. This is also done to track progress.
/// * The worker has finished ripping.
pub fn xmodits_subscription() -> Subscription<Message> {
    subscription::channel((), 1024, |mut output| async move {
        let mut state = State::Init;
        loop {
            match &mut state {
                State::Init => {
                    let (sender, receiver) = mpsc::channel::<Signal>(1);
                    state = State::Idle(receiver);

                    // It's important that this gets delivered, otherwise the program would be in an invalid state.
                    output
                        .try_send(Message::Ready(sender))
                        .expect("Sending a 'transmission channel' to main application.");
                }
                State::Idle(start_msg) => {
                    if let Some(config) = start_msg.recv().await {
                        let total = config.entries.len() as u64;
                        // let total = 0;
                        let (tx, rx) = mpsc::unbounded_channel();
                        info!("Started ripping");

                        // The ripping process is delegated by the subscription to a separate thread.
                        // This might not be idiomatic, but it works...
                        std::thread::spawn(move || {
                            todo!()
                            // let (paths, config) = config;
                            // super::extraction::rip(tx, paths, config);
                        });

                        state = State::Ripping {
                            ripping_msg: rx,
                            total,
                            progress: 0,
                            error_handler: ErrorHandler::default(),
                            total_errors: 0,
                            timer: Time::init(),
                        };
                    }
                }
                State::Ripping {
                    ripping_msg,
                    total_errors,
                    error_handler,
                    progress,
                    total,
                    timer,
                } => match ripping_msg.recv().await {
                    Some(ThreadMessage::Progress(error)) => {
                        *progress += 1;
                        let percentage: f32 = (*progress as f32 / *total as f32) * 100.0;

                        if let Some(failed) = error {
                            error!("{}", &failed);
                            *total_errors += 1;
                            error_handler.push(failed).await;
                        }

                        let _ = output.try_send(Message::Progress {
                            progress: percentage,
                            total_errors: *total_errors,
                        });
                    }
                    Some(ThreadMessage::SetTotal(new_total)) => {
                        *total = new_total;
                        *progress = 0;
                    }
                    Some(ThreadMessage::Info(info)) => {
                        let _ = output.try_send(Message::Info(info));
                    }
                    Some(ThreadMessage::Cancelled) => {
                        // let error = std::mem::take(error_handler);
                        timer.stop();

                        let msg = Message::Done {
                            state: CompleteState::Cancelled,
                            time: std::mem::take(timer),
                        };

                        output
                            .try_send(msg)
                            .expect("Sending 'extraction complete' message to application.");

                        info!("Cancelled!");
                        state = State::Init;
                    }
                    _ => {
                        timer.stop();
                        let error = std::mem::take(error_handler);

                        let msg = Message::Done {
                            state: CompleteState::from(error),
                            time: std::mem::take(timer),
                        };

                        // It's important that this gets delivered, otherwise the program would be in an invalid state.
                        output
                            .try_send(msg)
                            .expect("Sending 'extraction complete' message to application.");

                        info!("Done!");
                        state = State::Init;
                    }
                },
            }
        }
    })
}
