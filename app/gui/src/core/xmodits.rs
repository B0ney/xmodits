use anyhow::Result;
use tracing::info;
use std::path::{PathBuf, Path};
use std::time::Duration;
use xmodits_lib::{TrackerModule, Error};
use xmodits_lib::wav::Wav;
use xmodits_lib::load_module;
use super::cfg::Config;
// use iced::futures::channel::mpsc::{channel, Sender, Receiver, self};
use iced::{subscription, Subscription};
use tokio::task::spawn_blocking;
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Clone, Debug)]
pub enum DownloadMessage {
    Sender(Sender<DownloadMessage>),
    Done,
    Download,
    Cancel,
}

pub enum Progress {
    Cheese,
}
pub enum DownloadState {
    Starting,
    Idle {
        receiver: Receiver<DownloadMessage>,
    },
    Downloading {
        receiver: Receiver<DownloadMessage>,
        // query: VideoQuery,
        // downloader: Downloader,
    },
}

pub fn build_subscription() -> Subscription<DownloadMessage> {
    subscription::unfold(
        "Download",
        (DownloadState::Starting),
        |(state)| async move {
            match state {
                //? Create and pass sender to application
                DownloadState::Starting => {
                    let (sender, receiver) = mpsc::channel(1);

                    (
                        Some(DownloadMessage::Sender(sender.clone())),
                        DownloadState::Idle { receiver },
                    )
                }
                DownloadState::Idle { mut receiver} => {
                    let message = receiver.recv().await;
                    let (tx, rx) = mpsc::channel(1);
                    
                    std::thread::spawn(move || {
                        for _ in 0..20 {
                            std::thread::sleep(Duration::from_millis(1000));
                            tx.blocking_send(DownloadMessage::Download);
                        }
                        tx.blocking_send(DownloadMessage::Done);
                    });

                    info!("Received Message {message:?}");

                    match message {
                        Some(DownloadMessage::Download) => (
                            None,
                            DownloadState::Downloading {
                                receiver,
                                // query,
                                // downloader,
                            },
                        ),

                        _ => (None, DownloadState::Idle { receiver }),
                    }
                }

                DownloadState::Downloading {
                    mut receiver,
                    // query,
                    // downloader,
                } => {
                    info!("Test");
                    let message = receiver.recv().await;

                    match message {
                        Some(DownloadMessage::Done) => (
                            Some(DownloadMessage::Done),
                            DownloadState::Idle {
                                receiver,
                                // query,
                                // downloader,
                            },
                        ),
                        _ => (
                            Some(DownloadMessage::Done),
                            DownloadState::Downloading {
                                receiver,
                                // query,
                                // downloader,
                            },
                        ),
                    }
                }
            }
        },
    )
}
