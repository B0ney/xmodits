use super::cfg::SampleRippingConfig;
use super::extraction::{Failed, ThreadMsg};
use iced::{subscription, Subscription};
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::{self, Receiver, Sender, UnboundedReceiver};
use tracing::info;

pub type StartSignal = (Vec<PathBuf>, SampleRippingConfig);
use rand::Rng;

const ID: &str = "XMODITS_RIPPING";

/// State of subscription
#[derive(Default, Debug)]
enum State {
    #[default]
    Init,
    Idle(Receiver<StartSignal>),
    Ripping {
        ripping_msg: UnboundedReceiver<ThreadMsg>,
        total: usize,
        progress: usize,
        error_handler: Error,
        total_errors: usize,
    },
}

/// Messages emitted by subscription
#[derive(Clone, Debug)]
pub enum ExtractionMessage {
    Ready(Sender<StartSignal>),
    Done(CompleteState),
    Progress { progress: f32, total_errors: usize },
    Info(Option<String>),
}

#[derive(Default, Debug, Clone)]
pub enum CompleteState {
    #[default]
    NoErrors,
    SomeErrors(Vec<Failed>),
    TooMuchErrors {
        log: PathBuf,
        total: usize,
    },
    TooMuchErrorsNoLog {
        reason: String,
        errors: Vec<Failed>,
        discarded: usize,
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
/// * The sample extraction has completed
/// * A module has been ripped (can be used to track progress)
/// * A module cannot be ripped
pub fn xmodits_subscription() -> Subscription<ExtractionMessage> {
    subscription::unfold(ID, State::Init, |state| async move {
        match state {
            State::Init => {
                let (sender, receiver) = mpsc::channel::<StartSignal>(1);
                (
                    Some(ExtractionMessage::Ready(sender)),
                    State::Idle(receiver),
                )
            }
            State::Idle(mut start_msg) => match start_msg.recv().await {
                Some(config) => {
                    let total = config.0.len();
                    let (tx, rx) = mpsc::unbounded_channel();

                    std::thread::spawn(move || {
                        let (paths, config) = config;
                        super::extraction::rip(tx, paths, config);
                    });

                    (
                        None,
                        State::Ripping {
                            ripping_msg: rx,
                            total,
                            progress: 0,
                            error_handler: Error::default(),
                            total_errors: 0,
                        },
                    )
                }
                None => (None, State::Idle(start_msg)),
            },
            State::Ripping {
                mut ripping_msg,
                total,
                mut progress,
                mut error_handler,
                mut total_errors,
            } => match ripping_msg.recv().await {
                Some(ThreadMsg::Progress(error)) => {
                    progress += 1;
                    let percentage: f32 = (progress as f32 / total as f32) * 100.0;

                    if let Some(failed) = error {
                        info!("{}", &failed);
                        total_errors += 1;
                        error_handler.push(failed).await;
                    }

                    (
                        Some(ExtractionMessage::Progress {
                            progress: percentage,
                            total_errors,
                        }),
                        State::Ripping {
                            ripping_msg,
                            total,
                            progress,
                            error_handler,
                            total_errors,
                        },
                    )
                }
                Some(ThreadMsg::SetTotal(total)) => (
                    None,
                    State::Ripping {
                        ripping_msg,
                        total,
                        progress: 0,
                        error_handler,
                        total_errors,
                    },
                ),
                Some(ThreadMsg::Info(info)) => (
                    Some(ExtractionMessage::Info(info)),
                    State::Ripping {
                        ripping_msg,
                        total,
                        progress,
                        error_handler,
                        total_errors,
                    },
                ),
                _ => (
                    Some(ExtractionMessage::Done(CompleteState::from(error_handler))),
                    State::Init,
                ),
            },
        }
    })
}

const MAX: usize = 150;
const ABS_LIMIT: usize = MAX * 10;

#[derive(Debug)]
enum Error {
    Mem {
        errors: Vec<Failed>,
        log_dir: PathBuf,
    },
    File {
        total: usize,
        path: PathBuf,
        file: Box<tokio::fs::File>,
    },
    FailedFile {
        reason: String,
        errors: Vec<Failed>,
        discarded: usize,
    },
}

impl Default for Error {
    fn default() -> Self {
        Self::Mem {
            errors: Vec::new(),
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
                    .map(Box::new)
                {
                    Ok(mut file) => {
                        let lines = errors.len();

                        for error in errors {
                            Self::write_error(&mut file, error).await;
                        }

                        Self::File {
                            total: lines,
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

            Error::File {
                total: lines, file, ..
            } => {
                Self::write_error(file, error).await;
                *lines += 1;
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

    async fn write_error(file: &mut tokio::fs::File, error: Failed) {
        let failed_file = error.path.display().to_string();
        let _ = file.write_all(failed_file.as_bytes()).await;
        let _ = file.write_all(b"\n     ").await;
        let _ = file.write_all(error.reason.as_bytes()).await;
        let _ = file.write_all(b"\n\n").await;
    }
}
