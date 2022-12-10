use iced::{subscription, Subscription};
use std::path::PathBuf;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::{info, warn};
use xmodits_common::folder;

use super::cfg::SampleRippingConfig;
pub type gh = (Vec<PathBuf>, SampleRippingConfig);

/// Messages emitted by subscription
#[derive(Clone, Debug)]
pub enum DownloadMessage {
    Sender(Sender<gh>),
    Done,
    Progress,
    Error((PathBuf, String)),
    // Cancel,
}

/// Internal state of subscription
enum DownloadState {
    Starting,
    Idle {
        receiver: Receiver<gh>,
    },
    Downloading {
        ripping_msg: Receiver<DownloadMessage>,
    },
}

/// A subscription that allows the application to rip samples.
///
/// The subscription will emit messages when:
/// * The sample extraction has completed
/// * The module has been ripped (can be used to track progress)
/// * The module cannot be ripped
///
pub fn xmodits_subscription() -> Subscription<DownloadMessage> {
    subscription::unfold("Download", DownloadState::Starting, |state| async move {
        match state {
            //? Create and pass sender to application
            DownloadState::Starting => {
                let (sender, receiver) = mpsc::channel::<gh>(1);

                (
                    Some(DownloadMessage::Sender(sender)),
                    DownloadState::Idle { receiver },
                )
            }
            DownloadState::Idle {
                receiver: mut start_signal,
            } => {
                let message = start_signal.recv().await;

                info!("Received Message {message:?}");

                match message {
                    Some((paths, config)) => {
                        // The xmodits library is not async, so run it in a thread and have it communicate with the subscription.
                        // We spawn an ordinary thread instead of using "tokio::task::spawn_blocking" because
                        // it can be **easily** cancelled without stalling the async runtime.

                        let (tx, rx) = mpsc::channel(120);

                        std::thread::spawn(move || {
                            let dest_dir = &config.destination;
                            let namer = &config.naming.build_func();

                            info!("destination {}", &dest_dir.display());

                            for path in paths {
                                match xmodits_common::dump_samples_advanced(
                                    &path,
                                    &folder(dest_dir, &path, !config.no_folder),
                                    namer,
                                    !config.no_folder,
                                    &None,
                                    false,
                                ) {
                                    Ok(_) => {
                                        let _ = tx.blocking_send(DownloadMessage::Progress);
                                    }
                                    Err(e) => {
                                        warn!("{} <-- {}", &path.display(), e);

                                        let _ = tx.blocking_send(DownloadMessage::Error((
                                            path,
                                            e.to_string(),
                                        )));
                                    }
                                };
                            }

                            tx.blocking_send(DownloadMessage::Done);
                        });

                        (None, DownloadState::Downloading { ripping_msg: rx })
                    }

                    _ => (
                        None,
                        DownloadState::Idle {
                            receiver: start_signal,
                        },
                    ),
                }
            }

            DownloadState::Downloading {
                ripping_msg: mut receiver,
            } => {
                let message = receiver.recv().await;

                match message {
                    done @ Some(DownloadMessage::Done) => (done, DownloadState::Starting),
                    error @ Some(DownloadMessage::Error(_)) => (
                        error,
                        DownloadState::Downloading {
                            ripping_msg: receiver,
                        },
                    ),
                    _ => (
                        Some(DownloadMessage::Done),
                        DownloadState::Downloading {
                            ripping_msg: receiver,
                        },
                    ),
                }
            }
        }
    })
}
