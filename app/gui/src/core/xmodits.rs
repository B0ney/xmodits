use super::cfg::Config;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::info;
use xmodits_lib::load_module;
use xmodits_lib::wav::Wav;
use xmodits_lib::{Error, TrackerModule};
// use iced::futures::channel::mpsc::{channel, Sender, Receiver, self};
use iced::{subscription, Subscription};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task::spawn_blocking;

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
                DownloadState::Idle { mut receiver } => {
                    let message = receiver.recv().await;
                    let (tx, rx) = mpsc::channel(60);
                    
                    
                    info!("Received Message {message:?}");

                    match message {
                        Some(DownloadMessage::Download) => {
                            // Spawn blocking task, xmodits' ripping routine will start here
                            // TODO: have receiver for cancellation message
                            tokio::task::spawn_blocking(move || {
                                for _ in 0..10 {
                                    std::thread::sleep(Duration::from_millis(500));
                                    tx.blocking_send(DownloadMessage::Download).unwrap();
                                }
                                tx.blocking_send(DownloadMessage::Done).unwrap();
                            });

                            (
                                None,
                                DownloadState::Downloading {
                                    receiver: rx,
                                    // query,
                                    // downloader,
                                },
                            )
                        },

                        _ => (None, DownloadState::Idle { receiver }),
                    }
                }

                DownloadState::Downloading {
                    mut receiver,
                    // query,
                    // downloader,
                } => {
                    info!("Test");
                    let message = receiver.recv();

                    match message.await {
                        Some(DownloadMessage::Done) => (
                            Some(DownloadMessage::Done),
                            DownloadState::Starting,
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
