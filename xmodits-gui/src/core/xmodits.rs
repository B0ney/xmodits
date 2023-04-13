use super::cfg::SampleRippingConfig;
use super::extraction::{Failed, ThreadMsg};
use iced::subscription::channel;
use iced::{subscription, Subscription};
use rand::Rng;
use std::path::PathBuf;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::mpsc::{self, Receiver, Sender, UnboundedReceiver};
use tracing::info;

pub type StartSignal = (Vec<PathBuf>, SampleRippingConfig);

/// State of subscription
#[derive(Default, Debug)]
enum State {
    #[default]
    Init,
    Idle(Receiver<StartSignal>),
    Ripping {
        ripping_msg: UnboundedReceiver<ThreadMsg>,
        total: u64,
        progress: u64,
        error_handler: Error,
        total_errors: u64,
    },
}

/// Messages emitted by subscription
#[derive(Clone, Debug)]
pub enum ExtractionMessage {
    Ready(Sender<StartSignal>),
    Done(CompleteState),
    Progress { progress: f32, total_errors: u64 },
    Info(Option<String>),
}

#[derive(Default, Debug, Clone)]
pub enum CompleteState {
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
    },
}

impl From<Error> for CompleteState {
    fn from(value: Error) -> Self {
        match value {
            Error::Mem { errors, .. } => match errors.len() > 0 {
                true => Self::SomeErrors(errors),
                false => Self::NoErrors,
            },
            Error::File { total, path, .. } => Self::TooMuchErrors { log: path, total },
            Error::FailedFile {
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
pub fn xmodits_subscription() -> Subscription<ExtractionMessage> {
    subscription::channel((), 512, |mut output| async move {
        let mut state = State::Init;
        loop {
            match &mut state {
                State::Init => {
                    let (sender, receiver) = mpsc::channel::<StartSignal>(1);
                    state = State::Idle(receiver);
                    let _ = output.try_send(ExtractionMessage::Ready(sender));
                }
                State::Idle(start_msg) => match start_msg.recv().await {
                    Some(config) => {
                        let total = config.0.len() as u64;
                        let (tx, rx) = mpsc::unbounded_channel();

                        // The ripping process is delegated by the subscription to a separate thread.
                        // This might not be idiomatic, but it works...
                        std::thread::spawn(move || {
                            let (paths, config) = config;
                            super::extraction::rip(tx, paths, config);
                        });

                        state = State::Ripping {
                            ripping_msg: rx,
                            total,
                            progress: 0,
                            error_handler: Error::default(),
                            total_errors: 0,
                        };
                    }
                    None => (),
                },
                State::Ripping {
                    ripping_msg,
                    total_errors,
                    error_handler,
                    progress,
                    total,
                } => match ripping_msg.recv().await {
                    Some(ThreadMsg::Progress(error)) => {
                        *progress += 1;
                        let percentage: f32 = (*progress as f32 / *total as f32) * 100.0;

                        if let Some(failed) = error {
                            info!("{}", &failed);
                            *total_errors += 1;
                            error_handler.push(failed).await;
                        }

                        let _ = output.try_send(ExtractionMessage::Progress {
                            progress: percentage,
                            total_errors: *total_errors,
                        });
                    }
                    Some(ThreadMsg::SetTotal(new_total)) => {
                        *total = new_total;
                        *progress = 0;
                    }
                    Some(ThreadMsg::Info(info)) => {
                        let _ = output.try_send(ExtractionMessage::Info(info));
                    }
                    _ => {
                        let _ = output.try_send(ExtractionMessage::Done(CompleteState::from(
                            std::mem::take(error_handler),
                        )));
                        state = State::Init;
                    }
                },
            }
        }
    })
}

const MAX: usize = 150;
const ABS_LIMIT: usize = MAX * 10;

/// When the subscription receives errors from the workers, they're stored in this enum.
///
/// They're first stored in memory, but if there's too many of them to be displayed,
/// store them in a file.
/// At this stage, all future errors will be streamed to the file asynchronously.
///
/// However, if we can't create a file for some reason, we keep the errors in memory;
/// to preserve memory at this stage, future errors will be discarded when it's reached its absolute limit.
#[derive(Debug)]
enum Error {
    Mem {
        errors: Vec<Failed>,
        log_dir: PathBuf,
    },
    File {
        total: u64,
        path: PathBuf,
        file: Box<BufWriter<tokio::fs::File>>,
    },
    FailedFile {
        reason: String,
        errors: Vec<Failed>,
        discarded: u64,
    },
}

impl Default for Error {
    fn default() -> Self {
        Self::Mem {
            // Reserve an extra element so that pushing the last error before they're moved to a file
            // won't allocate an extra MAX elements
            errors: Vec::with_capacity(MAX + 1),
            log_dir: dirs::download_dir().expect("downloads folder"),
        }
    }
}

impl Error {
    async fn push(&mut self, error: Failed) {
        match self {
            Error::Mem { errors, log_dir } => {
                if errors.len() < MAX {
                    errors.push(error);
                    return;
                }

                let mut errors = std::mem::take(errors);
                let mut log_path = std::mem::take(log_dir);

                errors.push(error);
                log_path.push(format!(
                    "xmodits-error-log-{:04X}.txt",
                    rand::thread_rng().gen::<u16>()
                ));

                *self = match tokio::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&log_path)
                    .await
                    .map(BufWriter::new)
                    .map(Box::new)
                {
                    Ok(mut file) => {
                        let total = errors.len() as u64;

                        // Write stored errors to the new file
                        for error in errors {
                            Self::write_error(&mut file, error).await;
                        }

                        Self::File {
                            total,
                            path: log_path,
                            file,
                        }
                    }

                    Err(error) => Self::FailedFile {
                        reason: error.to_string(),
                        errors,
                        discarded: 0,
                    },
                };
            }

            Error::File { total, file, .. } => {
                Self::write_error(file, error).await;
                *total += 1;
            }

            Error::FailedFile {
                errors, discarded, ..
            } => {
                if errors.len() < ABS_LIMIT {
                    errors.push(error);
                    return;
                }
                *discarded += 1;
            }
        }
    }

    async fn write_error<W>(file: &mut W, error: Failed)
    where
        W: AsyncWriteExt + std::marker::Unpin,
    {
        let failed_file = error.path.display().to_string();
        let _ = file.write_all(failed_file.as_bytes()).await;
        let _ = file.write_all(b"\n     ").await;
        let _ = file.write_all(error.reason.as_bytes()).await;
        let _ = file.write_all(b"\n\n").await;
        let _ = file.flush().await;
    }
}
