use data::time::Time;

use iced::stream;
use iced::{futures::SinkExt, Subscription, advanced::subscription};
use std::{any::TypeId, path::PathBuf};
use tokio::sync::mpsc::{self, Receiver, Sender, UnboundedReceiver};
use tracing::{error, info};

pub use super::extraction::{self, ErrorHandler, Failed, Message as ThreadMessage, StopMessage};
use super::stop_flag::{self, StopFlag};
use super::Signal;

/// Messages emitted by subscription
#[derive(Clone, Debug)]
pub enum Message {
    Ready(Sender<Signal>),
    Progress {
        progress: f32,
        errors: u64,
    },
    Done {
        state: CompleteState,
        time: Time,
        destination: PathBuf,
    },
    Info(Option<String>),
}

#[derive(Default, Debug, Clone)]
pub enum CompleteState {
    #[default]
    NoErrors,
    Cancelled,
    Aborted,
    SomeErrors(Vec<Failed>),
    TooMuchErrors {
        log: PathBuf,
        total: u64,
    },
    TooMuchErrorsNoLog {
        reason: String,
        errors: Vec<Failed>,
        discarded: u64,
    },
}

impl CompleteState {
    pub fn errors_ref(&self) -> Option<&Vec<Failed>> {
        match self {
            Self::SomeErrors(errors) | Self::TooMuchErrorsNoLog { errors, .. } => Some(errors),
            _ => None,
        }
    }
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
pub fn subscription() -> Subscription<Message> {
    struct Ripper;

    Subscription::run_with_id(TypeId::of::<Ripper>(), stream::channel(4096, |mut output| async move {
        enum State {
            Init,
            Idle(Receiver<Signal>),
            Ripping {
                ripping_msg: UnboundedReceiver<ThreadMessage>,
                total: u64,
                progress: u64,
                error_handler: ErrorHandler,
                total_errors: u64,
                timer: Time,
                destination: PathBuf,
            },
        }

        let mut state = State::Init;

        loop {
            match &mut state {
                State::Init => {
                    stop_flag::reset();

                    let (sender, receiver) = mpsc::channel::<Signal>(1);
                    state = State::Idle(receiver);

                    // It's important that this gets delivered, otherwise the program would be in an invalid state.
                    output
                        .send(Message::Ready(sender))
                        .await
                        .expect("Sending a 'transmission channel' to main application.");
                }
                State::Idle(start_msg) => {
                    if let Some(config) = start_msg.recv().await {
                        let total = config.entries.len() as u64;
                        let destination = config.ripping.destination.clone();
                        let (tx, rx) = mpsc::unbounded_channel();

                        // The ripping process is delegated by the subscription to a separate thread.
                        // This might not be idiomatic, but it works...
                        std::thread::spawn(move || {
                            info!("Started ripping");
                            extraction::rip(tx, config);
                        });

                        state = State::Ripping {
                            ripping_msg: rx,
                            total,
                            progress: 0,
                            error_handler: ErrorHandler::new(destination.clone()),
                            total_errors: 0,
                            timer: Time::init(),
                            destination,
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
                    destination,
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
                            errors: *total_errors,
                        });
                    }
                    Some(ThreadMessage::SetTotal(new_total)) => {
                        *total = new_total;
                        *progress = 0;
                    }
                    Some(ThreadMessage::Info(info)) => {
                        let _ = output.try_send(Message::Info(info));
                    }
                    Some(ThreadMessage::Stop(stop)) => {
                        timer.stop();

                        let completed_state = match stop {
                            StopMessage::Abort => CompleteState::Aborted,
                            StopMessage::Cancel => CompleteState::Cancelled,
                        };

                        let msg = Message::Done {
                            state: completed_state,
                            time: std::mem::take(timer),
                            destination: std::mem::take(destination),
                        };

                        info!("Cancelled!");
                        output
                            .send(msg)
                            .await
                            .expect("Sending 'extraction complete' message to application.");

                        state = State::Init;
                    }
                    Some(ThreadMessage::Done) => {
                        timer.stop();
                        let error = std::mem::take(error_handler);

                        let msg = Message::Done {
                            state: CompleteState::from(error),
                            time: std::mem::take(timer),
                            destination: std::mem::take(destination),
                        };

                        // It's important that this gets delivered, otherwise the program would be in an invalid state.
                        output
                            .send(msg)
                            .await
                            .expect("Sending 'extraction complete' message to application.");

                        info!("Done!");
                        state = State::Init;
                    }
                    None => {
                        timer.stop();
                        let error = std::mem::take(error_handler);

                        let completed_state: CompleteState = match stop_flag::get_flag() {
                            StopFlag::None => CompleteState::from(error),
                            StopFlag::Cancel => CompleteState::Cancelled,
                            StopFlag::Abort => CompleteState::Aborted,
                        };

                        let msg = Message::Done {
                            state: completed_state,
                            time: std::mem::take(timer),
                            destination: std::mem::take(destination),
                        };

                        
                        tracing::error!("Lost communication with the workers. This usually means something bad happened...");

                        // It's important that this gets delivered, otherwise the program would be in an invalid state.
                        output
                            .send(msg)
                            .await
                            .expect("Sending 'extraction complete' message to application.");

                        state = State::Init;
                    }
                },
            }
        }
    }))
}
