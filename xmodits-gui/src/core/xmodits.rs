use iced::{subscription, Subscription};
use std::path::PathBuf;
use tokio::sync::mpsc::{self, Receiver, Sender, UnboundedReceiver, UnboundedSender};

use super::cfg::SampleRippingConfig;
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
    },
}

/// Messages emitted by subscription
#[derive(Clone, Debug)]
pub enum DownloadMessage {
    Ready(Sender<StartSignal>),
    Done,
    Progress {
        progress: f32,
        error: Option<Failed>,
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

                    spawn_worker_thread(tx, config);
                    (
                        None,
                        State::Ripping {
                            ripping_msg: rx,
                            total,
                            progress: 0,
                        },
                    )
                }
                None => (None, State::Idle(start_msg)),
            },
            State::Ripping {
                mut ripping_msg,
                total,
                mut progress,
            } => match ripping_msg.recv().await {
                Some(ThreadMsg::Progress(error)) => {
                    progress += 1;
                    let percentage: f32 = (progress as f32 / total as f32) * 100.0;
                    (
                        Some(DownloadMessage::Progress {
                            progress: percentage,
                            error,
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
                        progress: 0,
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
    })
}

// async fn rip(state: State) -> (Option<DownloadMessage>, State) {

// }

fn spawn_worker_thread(tx: UnboundedSender<ThreadMsg>, config: StartSignal) {
    std::thread::spawn(move || {
        let (paths, config) = config;
        super::extraction::rip(tx, paths, config);
    });
}
