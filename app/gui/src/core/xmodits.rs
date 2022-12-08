use super::cfg::Config;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::{info, warn};
use xmodits_lib::{load_module, SampleNamerFunc};
use xmodits_lib::wav::Wav;
use xmodits_lib::{XmoditsError, TrackerModule};
use xmodits_common::{dump_samples_advanced, folder};
// use iced::futures::channel::mpsc::{channel, Sender, Receiver, self};
use iced::{subscription, Subscription};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task::spawn_blocking;

#[derive(Clone, Debug)]
pub enum DownloadMessage {
    Sender(Sender<DownloadMessage>),
    Done,
    Download((Vec<PathBuf>, Config)),
    Progress,
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
                        Some(DownloadMessage::Download((paths, config))) => {
                            // Spawn blocking task, xmodits' ripping routine will start here
                            // TODO: have receiver for cancellation message
                            // spawn ordinary thread because it can easily be cancelled without stalling the async runtime
                            // the thread can then send messages to the subscription
                            // 
                            std::thread::spawn(move || {
                                use std::cmp;
                                let dest_dir = &config.destination;
                                let namer = &config.build_func();
                                let mut errors: Vec<XmoditsError> = Vec::new();
                                info!("destination {}", &dest_dir.display());

                                for path in paths {
                                    if let Err(e) = xmodits_common::dump_samples_advanced(
                                            &path,
                                            &folder(dest_dir, &path, !config.no_folder),
                                            namer,
                                            !config.no_folder,
                                            &None,
                                            false
                                        ) {
                                            warn!("{}", e);
                                    };
                                    tx.blocking_send(DownloadMessage::Progress);
                                }
   
                                tx.blocking_send(DownloadMessage::Done);
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
