use iced::{subscription, Subscription};
use std::path::PathBuf;
use tokio::sync::mpsc::{self, Receiver, Sender, UnboundedReceiver, UnboundedSender};

use super::cfg::{self, Config, SampleRippingConfig};
pub type StartSignal = (Vec<PathBuf>, SampleRippingConfig);
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
        errors: usize,
    },
}

/// Messages emitted by subscription
#[derive(Clone, Debug)]
pub enum DownloadMessage {
    Ready(Sender<StartSignal>),
    Done,
    Progress {
        progress: f32,
        errors: usize,
        // error: Option<Failed>,
    },
    Info(Option<String>),
}

use super::extraction::{Failed, ThreadMsg};

/// The subscription will emit messages when:
/// * The sample extraction has completed
/// * A module has been ripped (can be used to track progress)
/// * A module cannot be ripped
pub fn xmodits_subscription() -> Subscription<DownloadMessage> {
    subscription::unfold(ID, State::Init, |state| async move {
        match state {
            State::Init => {
                let (sender, receiver) = mpsc::channel::<StartSignal>(1);
                (Some(DownloadMessage::Ready(sender)), State::Idle(receiver))
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
                            errors: 0,
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
                mut errors,
            } => match ripping_msg.recv().await {
                Some(ThreadMsg::Progress(error)) => {
                    progress += 1;
                    let percentage: f32 = (progress as f32 / total as f32) * 100.0;

                    if let Some(failed) = error {
                        errors += 1;
                        error_handler.push(failed).await;
                    }

                    (
                        Some(DownloadMessage::Progress {
                            progress: percentage,
                            errors,
                        }),
                        State::Ripping {
                            ripping_msg,
                            total,
                            progress,
                            error_handler,
                            errors,
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
                        errors,
                    },
                ),
                Some(ThreadMsg::Info(info)) => (
                    Some(DownloadMessage::Info(info)),
                    State::Ripping {
                        ripping_msg,
                        total,
                        progress,
                        error_handler,
                        errors,
                    },
                ),
                Some(ThreadMsg::Done) => {
                    (Some(DownloadMessage::Done), State::Init)
                }
                // None => todo!(),
                _ => (Some(DownloadMessage::Done), State::Init),
            },
        }
    })
}

const MAX: usize = 150;
const ABS_LIMIT: usize = MAX * 10;

#[derive(Debug)]
pub enum Error {
    Mem {
        errors: Vec<Failed>,
        delegate: PathBuf,
    },
    File {
        lines: usize,
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
            delegate: dirs::download_dir().expect("downloads folder"),
        }
    }
}
use tokio::io::AsyncWriteExt;

impl Error {
    async fn push(&mut self, error: Failed) {
        match self {
            Error::Mem { errors, delegate } => {
                if errors.len() < MAX {
                    errors.push(error);
                    return;
                }

                let mut errors = std::mem::take(errors);
                let mut folder = std::mem::take(delegate);

                errors.push(error);
                folder.push("xmodits-error-log.txt");

                *self = match tokio::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&folder)
                    .await
                    .map(|f| Box::new(f))
                {
                    Ok(mut file) => {
                        let lines = errors.len();

                        for error in errors {
                            Self::write_error(&mut file, error).await;
                        }

                        Self::File {
                            lines,
                            path: folder,
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

            Error::File { lines, file, .. } => {
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
        let _ = file.write_all(b"\n").await;
    }
}
