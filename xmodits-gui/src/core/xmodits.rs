use iced::{subscription, Subscription};
use std::path::PathBuf;
use tokio::sync::mpsc::{self, Receiver, Sender};
use walkdir::WalkDir;

use xmodits_lib::{
    common::extract, fmt::loader::load_module, interface::ripper::Ripper, SampleNamer,
    SampleNamerTrait,
};
// use xmodits_lib::common::folder;

use super::cfg::SampleRippingConfig;
pub type StartSignal = (Vec<PathBuf>, SampleRippingConfig);
const ID: &str = "XMODITS_RIPPING";

/// State of subscription
#[derive(Default, Debug)]
enum State {
    #[default]
    Init,
    Idle {
        start_msg: Receiver<StartSignal>,
    },
    Ripping {
        ripping_msg: Receiver<ThreadMsg>,
        total: usize,
        progress: usize,
    },
}

/// Messages emitted by subscription
#[derive(Clone, Debug)]
pub enum DownloadMessage {
    Ready(Sender<StartSignal>),
    Done,
    Progress {
        progress: f32,
        result: Result<(), (PathBuf, String)>,
    },
    Info(Option<String>),
}

/// Messages emitted by thread
#[derive(Debug)]
enum ThreadMsg {
    SetTotal(usize),
    Ok,
    Failed((PathBuf, String)),
    Done,
    Info(Option<String>),
}

/// The subscription will emit messages when:
/// * The sample extraction has completed
/// * A module has been ripped (can be used to track progress)
/// * A module cannot be ripped
pub fn xmodits_subscription() -> Subscription<DownloadMessage> {
    subscription::unfold(ID, State::Init, rip)
}

async fn rip(state: State) -> (Option<DownloadMessage>, State) {
    match state {
        State::Init => {
            let (sender, receiver) = mpsc::channel::<StartSignal>(1);
            (
                Some(DownloadMessage::Ready(sender)),
                State::Idle {
                    start_msg: receiver,
                },
            )
        }
        State::Idle { mut start_msg } => match start_msg.recv().await {
            Some(config) => {
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
            None => (None, State::Idle { start_msg }),
        },
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
            Some(ThreadMsg::SetTotal(total)) => (
                None,
                State::Ripping {
                    ripping_msg,
                    total,
                    progress,
                },
            ),
            Some(ThreadMsg::Info(info)) => (
                Some(DownloadMessage::Info(info)),
                State::Ripping {
                    ripping_msg,
                    total,
                    progress,
                },
            ),
            _ => (Some(DownloadMessage::Done), State::Init),
        },
    }
}

fn spawn_thread(tx: Sender<ThreadMsg>, config: StartSignal) {
    std::thread::spawn(move || {
        let (paths, config) = config;

        let dest_dir = config.destination;

        if !dest_dir.is_dir() {
            if let Err(e) = std::fs::create_dir(&dest_dir) {
                tx.blocking_send(ThreadMsg::Failed((dest_dir, e.to_string())))
                    .expect("Channel closed prematurely");

                tx.blocking_send(ThreadMsg::Done)
                    .expect("Channel closed prematurely");
                return;
            };
        }

        let scan_depth = match config.folder_recursion_depth {
            0 => 1,
            d => d,
        };

        let mut files: Vec<PathBuf> = Vec::new();
        let mut folders: Vec<PathBuf> = Vec::new();

        for i in paths {
            if i.is_file() {
                files.push(i)
            } else if i.is_dir() {
                folders.push(i)
            }
        }

        tx.blocking_send(ThreadMsg::Info(Some("Traversing folders...".into())))
            .expect("Channel closed prematurely");

        // Can use a lot of memory if max_depth is too high
        let expanded_folders = folders.into_iter().flat_map(move |f| {
            WalkDir::new(f)
                .max_depth(scan_depth as usize)
                .into_iter()
                .filter_map(|f| match f.ok() {
                    Some(d) if d.path().is_file() => Some(d.into_path()),
                    _ => None,
                })
        });

        // Collect because we should inform the user how many files it's ripping
        let expanded_paths: Vec<PathBuf> = files.into_iter().chain(expanded_folders).collect();

        tx.blocking_send(ThreadMsg::SetTotal(expanded_paths.len()))
            .expect("Channel closed prematurely");

        tx.blocking_send(ThreadMsg::Info(Some(format!(
            "Ripping {} files...",
            expanded_paths.len()
        ))))
        .expect("Channel closed prematurely");

        let ripper = Ripper::new(
            config.naming.build_func(),
            config.exported_format.into(),
        );
        // ripper.change_namer(config.naming.build_func());

        for path in expanded_paths {
            tx.blocking_send(
                match extract(
                    &path,
                    &dest_dir,
                    &ripper,
                    !config.no_folder
                ) {
                    Ok(_) => ThreadMsg::Ok,
                    Err(e) => ThreadMsg::Failed((path, e.to_string())),
                },
            )
            .expect("Channel closed prematurely");
        }

        tx.blocking_send(ThreadMsg::Done)
            .expect("Channel closed prematurely");
    });
}
