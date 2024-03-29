pub mod buffer;
pub mod error;
pub mod error_handler;

pub use buffer::{Batch, Buffer};
pub use error::Failed;
pub use error_handler::ErrorHandler;

use crate::logger;

use super::stop_flag;
use super::Signal;

use data::config::SampleRippingConfig;
use xmodits_lib::Ripper;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Seek, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender as AsyncSender;
use walkdir::WalkDir;

#[derive(Debug)]
pub enum Message {
    SetTotal(u64),
    Info(Option<String>),
    Progress(Option<Failed>),
    Done,
    Stop(StopMessage),
}

#[derive(Debug)]
pub enum StopMessage {
    Cancel,
    Abort,
}

impl Message {
    pub fn info(str: impl Into<String>) -> Self {
        Self::Info(Some(str.into()))
    }
}

fn split_files_folders(paths: Vec<PathBuf>) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut folders: Vec<PathBuf> = Vec::new();

    paths.into_iter().for_each(|f| match f.is_file() {
        true => files.push(f),
        false => folders.push(f),
    });

    (files, folders)
}

pub fn rip(tx: AsyncSender<Message>, signal: Signal) {
    let (files, folders) = split_files_folders(signal.entries);

    let mut cfg = signal.ripping;

    cfg.folder_max_depth = match cfg.folder_max_depth {
        0 => 1,
        d => d,
    };

    let ripper = Arc::new(Ripper::new(
        signal.naming.build_func(),
        cfg.exported_format.into(),
    ));

    // Create the destination folder if it doesn't exist
    let _ = std::fs::create_dir(&cfg.destination);

    stage_1(tx.clone(), files, ripper.clone(), &cfg);
    stage_2(tx.clone(), folders, ripper, cfg);

    tx.send(match stop_flag::get_flag() {
        stop_flag::StopFlag::None => Message::Done,
        stop_flag::StopFlag::Cancel => Message::Stop(StopMessage::Cancel),
        stop_flag::StopFlag::Abort => Message::Stop(StopMessage::Abort),
    })
    .expect("Informing main GUI that the extraction has completed");
}

fn stage_1(
    subscr_tx: AsyncSender<Message>,
    files: Vec<PathBuf>,
    ripper: Arc<Ripper>,
    cfg: &SampleRippingConfig,
) {
    if files.is_empty() {
        return;
    }
    subscr_tx
        .send(Message::SetTotal(files.len() as u64))
        .unwrap();

    let info = format!("Stage 1: Ripping {} files...", files.len());
    subscr_tx.send(Message::info(info)).unwrap();

    let filter = strict_loading(cfg.strict);

    for file in files.iter().filter(|f| filter(f)) {
        if stop_flag::is_set() {
            break;
        }

        let _ = subscr_tx.send(Message::Progress(
            extract(file, &cfg.destination, ripper.as_ref(), cfg.self_contained)
                .map_err(|error| Failed::new(file.display().to_string(), error))
                .err(),
        ));
    }
}

/// todo add documentation
///
fn stage_2(
    subscr_tx: AsyncSender<Message>,
    folders: Vec<PathBuf>,
    ripper: Arc<Ripper>,
    cfg: SampleRippingConfig,
) {
    if folders.is_empty() || stop_flag::is_set() {
        return;
    }
    let selected_dirs = folders.len();
    subscr_tx
        .send(Message::info("Traversing Directories..."))
        .unwrap();

    let filter = strict_loading(cfg.strict);

    let (mut file, lines) = traverse(folders, cfg.folder_max_depth, filter, |lines| {
        let info = format!("Traversing Directories...\n({lines} filtered files)");
        subscr_tx.send(Message::info(info)).unwrap()
    });

    subscr_tx.send(Message::SetTotal(lines)).unwrap();

    let plural = |n: u64| -> &str {
        if n > 1 {
            "s"
        } else {
            ""
        }
    };

    let info = format!(
        "Stage 2: Ripping {lines} file{} from {selected_dirs} folder{}...",
        plural(lines),
        plural(selected_dirs as u64)
    );
    subscr_tx.send(Message::info(info)).unwrap();

    if stop_flag::is_set() {
        return;
    }

    Batcher::new(&mut file, batch_size(lines), ripper, cfg, subscr_tx).start();
}

fn batch_size(lines: u64) -> usize {
    match lines {
        x if x <= 128 => 64,
        x if x <= 256 => 128,
        x if x <= 512 => 256,
        x if x <= 1024 => 512,
        x if x <= 2048 => 1024,
        _ => 2048,
    }
}

fn extract(
    file: impl AsRef<Path>,
    destination: &Path,
    ripper: &Ripper,
    self_contained: bool,
) -> Result<(), xmodits_lib::Error> {
    logger::log_file_on_panic(file.as_ref(), |file| {
        xmodits_lib::extract(file, destination, ripper, self_contained)
    })
}

/// Traversing deeply nested directories can use a lot of memory.
///
/// For that reason we write the output to a file
fn traverse(
    dirs: Vec<PathBuf>,
    max_depth: u8,
    filter: impl Fn(&Path) -> bool,
    callback: impl Fn(u64),
) -> (BufReader<File>, u64) {
    let mut file = tempfile::tempfile()
        .map(BufWriter::new)
        .expect("Creating a temporary file");

    // store the number of entries
    let mut lines: u64 = 0;

    // traverse list of directories, output to a file
    'traversal: for folder in dirs.into_iter() {
        for entry in WalkDir::new(folder)
            .max_depth(max_depth as usize)
            .into_iter()
        {
            if stop_flag::is_set() {
                break 'traversal;
            }

            let Ok(f) = entry else {
                continue;
            };

            if f.path().is_file() && filter(f.path()) {
                lines += 1;
                callback(lines);
                file.write_fmt(format_args!("{}\n", f.path().display()))
                    .expect("Writing file entry");
            }
        }
    }
    let _ = file.flush();
    let (mut file, _) = file.into_parts();

    // Rewind cursor to beginning
    file.rewind().unwrap();

    // Wrap file in bufreader and return
    (BufReader::new(file), lines)
}

#[derive(Copy, Clone)]
struct NextBatch;

#[derive(Default)]
struct State {
    pub complete: bool,
}

///
///
struct Batcher<'io> {
    file: &'io mut BufReader<File>,
    batch_size: usize,
    batch_number: usize,
    state: State,
    buffer: Buffer<String>,
    batch_tx: Sender<Batch<String>>,
    worker_rx: Receiver<NextBatch>,
}

impl<'io> Batcher<'io> {
    fn new(
        file: &'io mut BufReader<File>,
        batch_size: usize,
        ripper: Arc<Ripper>,
        cfg: SampleRippingConfig,
        subscr_tx: AsyncSender<Message>,
    ) -> Batcher<'io> {
        let (batch_tx, batch_rx) = mpsc::channel::<Batch<String>>();
        let (worker_tx, worker_rx) = mpsc::channel::<NextBatch>();

        let mut batcher = Self {
            file,
            batch_size,
            batch_number: 0,
            state: State::default(),
            buffer: Buffer::init(batch_size),
            batch_tx,
            worker_rx,
        };

        // load first buffer
        batcher.load();

        // Spawn workers
        {
            use rayon::prelude::*;

            let destination = cfg.destination;
            let self_contained = cfg.self_contained;

            rayon::ThreadPoolBuilder::new()
                .thread_name(|index| format!("XMODITS Ripping Thread - {index}"))
                .num_threads(cfg.worker_threads)
                .panic_handler(|_| {/* Don't abort process */})
                .build()
                .expect("constructing thread pool")
                .spawn(move || {
                    while let Ok(batch) = batch_rx.recv() {
                        batch.lock().par_iter().for_each(|file| {
                            if stop_flag::is_set() {
                                return;
                            }

                            // Send an update to the subscription
                            let _ = subscr_tx.send(Message::Progress(
                                extract(file, &destination, &ripper, self_contained)
                                    .err()
                                    .map(|error| Failed::new(file.into(), error)),
                            ));
                        });

                        // Tell the batcher we're done so that it can send the next round
                        let _ = worker_tx.send(NextBatch);
                    }
                });
        };

        batcher
    }

    pub fn start(&mut self) {
        let mut is_last_batch = false;

        while !(self.state.complete || stop_flag::is_set()) {
            // If this is the last batch, set the state to complete
            // and send the last batch. When complete this loop terminates.
            self.state.complete = is_last_batch;

            // Send the current batch to the worker thread
            self.batch_tx.send(self.buffer.current_buffer()).unwrap();

            // While the worker thread is dealing with the first batch,
            // prepare the next batch. Ping-pong buffering ftw.
            is_last_batch = self.load_next_batch();

            // wait for the worker to finish, then loop
            match self.worker_rx.recv() {
                Ok(NextBatch) => continue,
                Err(_) => break, // TODO
            }
        }
    }

    /// Load the next batch of lines
    pub fn load_next_batch(&mut self) -> bool {
        self.buffer.switch();
        self.load()
    }

    /// Store ``batch_size`` lines to the current buffer
    ///
    /// Returns true if the buffer is less than the defined batch size.
    ///
    /// This indicates we've reached the end of the file.
    pub fn load(&mut self) -> bool {
        // Acquire buffer
        let buffer = self.buffer.current_buffer();
        let mut buffer = buffer.lock();

        // Clear the buffer.
        buffer.clear();

        // Store the read lines into the buffer.
        // The buffer has a batch_size capacity so
        // it won't re-allocate
        self.file
            .lines()
            .take(self.batch_size)
            .filter_map(|f| f.ok())
            .for_each(|line| buffer.push(line));

        self.batch_number += 1;

        // Have a way of notifying the caller that this is is the last batch,
        // and should not be called again.
        buffer.len() < self.batch_size
    }
}

pub fn strict_loading(strict: bool) -> impl Fn(&Path) -> bool {
    match strict {
        true => move |path: &Path| {
            const EXT: &[&str] = &[
                "it", "xm", "s3m", "mod", "umx", "mptm", "IT", "XM", "S3M", "MOD", "UMX", "MPTM",
            ];

            let Some(ext) = path.extension().and_then(|f| f.to_str()) else {
                return false;
            };

            EXT.contains(&ext)
        },

        false => |_: &Path| true,
    }
}
