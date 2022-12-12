use iced::{subscription, Subscription};
use walkdir::WalkDir;
use std::path::PathBuf;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::{info, warn};
use xmodits_common::folder;

use super::cfg::SampleRippingConfig;
pub type gh = (Vec<PathBuf>, SampleRippingConfig, u8);
const ID: &str = "XMODITS_RIPPING";

/// State of subscription
#[derive(Default, Debug)]
enum State {
    #[default]
    Init,
    Idle {
        start_msg: Receiver<gh>,
    },
    Start(gh),
    Ripping {
        ripping_msg: Receiver<ThreadMsg>,
        total: usize,
        progress: usize,
    },
    Done,
}

/// Messages emitted by subscription
#[derive(Clone, Debug)]
pub enum DownloadMessage {
    Ready(Sender<gh>),
    Done,
    Progress {
        progress: f32,
        result: Result<(), (PathBuf, String)>,
    },
    // Cancel,
}

/// Messages emitted by thread
enum ThreadMsg {
    Ok,
    Failed((PathBuf, String)),
    Done,
}

/// The subscription will emit messages when:
/// * The sample extraction has completed
/// * A module has been ripped (can be used to track progress)
/// * A module cannot be ripped
pub fn xmodits_subscription() -> Subscription<DownloadMessage> {
    subscription::unfold(ID, State::Init, |state| rip(state))
}

async fn rip(state: State) -> (Option<DownloadMessage>, State) {
    match state {
        State::Init => {
            let (sender, receiver) = mpsc::channel::<gh>(1);
            (
                Some(DownloadMessage::Ready(sender)),
                State::Idle {
                    start_msg: receiver,
                },
            )
        }
        State::Idle { mut start_msg } => match start_msg.recv().await {
            Some(gh) => (None, State::Start(gh)),
            None => (None, State::Idle { start_msg }),
        },
        State::Start(config) => {
            let total = config.0.len();
            let (tx, rx) = mpsc::channel(120);

            spawn_thread(tx, config);
            (
                None,
                State::Ripping {
                    ripping_msg: rx,
                    total,
                    progress: 0,
                },
            )
        }
        State::Ripping {
            mut ripping_msg,
            total,
            mut progress,
        } => match ripping_msg.recv().await {
            Some(result @ (ThreadMsg::Ok | ThreadMsg::Failed(_))) => {
                progress += 1;
                let percentage: f32 = (progress as f32 / total as f32) * 100.0;
                let result = match result {
                    ThreadMsg::Ok => Ok(()),
                    ThreadMsg::Failed(err) => Err(err),
                    _ => unreachable!(),
                };
                (
                    Some(DownloadMessage::Progress {
                        progress: percentage,
                        result,
                    }),
                    State::Ripping {
                        ripping_msg,
                        total,
                        progress,
                    },
                )
            }
            _ => (Some(DownloadMessage::Done), State::Init),
        },

        _ => (Some(DownloadMessage::Done), State::Init),
    }
}

fn spawn_thread(tx: Sender<ThreadMsg>, config: gh) {
    let (paths, config, mut scan_depth) = config;
    if scan_depth == 0 { scan_depth += 1 }

    let mut files: Vec<PathBuf> = Vec::new();
    let mut folders: Vec<PathBuf> = Vec::new();

    for i in paths {
        if i.is_file() {
            files.push(i)
        } else if i.is_dir() {
            folders.push(i)
        }
    };

    let expanded_folders = folders
        .into_iter()
        .map(move |f| WalkDir::new(f)
            .max_depth(scan_depth as usize)
            .into_iter()
            .filter_map(|f| f.ok())
            .map(|f|f.into_path())
            .filter(|f| f.is_file())
        )
        .flatten();
    
    let expanded_paths = files
        .into_iter()
        .chain(expanded_folders);

    std::thread::spawn(move || {
        let dest_dir = &config.destination;
        let namer = &config.naming.build_func();
        
        info!("{}", dest_dir.display());

        for path in expanded_paths {
            match xmodits_common::dump_samples_advanced(
                &path,
                &folder(dest_dir, &path, !config.no_folder),
                namer,
                !config.no_folder,
                &None,
                false,
            ) {
                Ok(_) => {
                    let _ = tx.blocking_send(ThreadMsg::Ok);
                }
                Err(e) => {
                    let _ = tx.blocking_send(ThreadMsg::Failed((path, e.to_string())));
                }
            };
        }

        tx.blocking_send(ThreadMsg::Done);
    });
}
