use crate::core::cfg::SampleRippingConfig;
use crate::core::track::GLOBAL_TRACKER;
use crate::gui::utils::file_name;

use std::fs::File;
use std::io::{BufRead, BufReader, Seek, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};

use parking_lot::Mutex;
use tokio::sync::mpsc::UnboundedSender as AsyncSender;
use walkdir::WalkDir;
use xmodits_lib::common::extract;
use xmodits_lib::interface::ripper::Ripper;

use super::xmodits::cancelled;

#[derive(Debug)]
pub enum ThreadMsg {
    SetTotal(u64),
    Info(Option<String>),
    Progress(Option<Failed>),
    Done,
    Cancelled,
}

impl ThreadMsg {
    pub fn info(str: &str) -> Self {
        Self::Info(Some(str.to_owned()))
    }
}

pub fn rip(tx: AsyncSender<ThreadMsg>, paths: Vec<PathBuf>, mut cfg: SampleRippingConfig) {
    GLOBAL_TRACKER.reset();

    // split files and folders
    let mut files: Vec<PathBuf> = Vec::new();
    let mut folders: Vec<PathBuf> = Vec::new();

    for i in paths {
        if i.is_file() {
            files.push(i)
        } else if i.is_dir() {
            folders.push(i)
        }
    }

    cfg.folder_max_depth = match cfg.folder_max_depth {
        0 => 1,
        d => d,
    };

    let ripper = Arc::new(Ripper::new(
        cfg.naming.build_func(),
        cfg.exported_format.into(),
    ));

    // Create the destination folder if it doesn't exist
    let _ = std::fs::create_dir(&cfg.destination);

    stage_1(tx.clone(), files, ripper.clone(), &cfg);
    stage_2(tx.clone(), folders, ripper, cfg);

    tx.send(match cancelled() {
        true => ThreadMsg::Cancelled,
        false => ThreadMsg::Done,
    })
    .expect("Informing main GUI that the extraction has completed");
}

fn stage_1(
    subscr_tx: AsyncSender<ThreadMsg>,
    files: Vec<PathBuf>,
    ripper: Arc<Ripper>,
    cfg: &SampleRippingConfig,
) {
    if files.is_empty() {
        return;
    }
    subscr_tx
        .send(ThreadMsg::SetTotal(files.len() as u64))
        .unwrap();

    subscr_tx
        .send(ThreadMsg::Info(Some(format!(
            "Stage 1: Ripping {} files...",
            files.len()
        ))))
        .unwrap();

    let files = GLOBAL_TRACKER.set_files(files);
    let filter = strict_loading(cfg.strict);

    for file in files.lock().iter().filter(|f| filter(f)) {
        if cancelled() {
            break;
        }

        let _ = subscr_tx.send(ThreadMsg::Progress(
            extract(&file, &cfg.destination, ripper.as_ref(), cfg.self_contained)
                .map_err(|error| Failed::new(file.display().to_string(), error))
                .err(),
        ));

        GLOBAL_TRACKER.incr_file();
    }
}

/// todo add documentation
///
fn stage_2(
    subscr_tx: AsyncSender<ThreadMsg>,
    folders: Vec<PathBuf>,
    ripper: Arc<Ripper>,
    cfg: SampleRippingConfig,
) {
    if folders.is_empty() || cancelled() {
        return;
    }
    let selected_dirs = folders.len();
    subscr_tx
        .send(ThreadMsg::info("Traversing Directories..."))
        .unwrap();

    let filter = strict_loading(cfg.strict);

    let (mut file, lines) = traverse(folders, cfg.folder_max_depth, filter, |lines| {
        subscr_tx
            .send(ThreadMsg::Info(Some(format!(
                "Traversing Directories...\n({lines} filtered files)"
            ))))
            .unwrap()
    });

    subscr_tx.send(ThreadMsg::SetTotal(lines)).unwrap();

    subscr_tx
        .send(ThreadMsg::Info(Some(format!(
            "Stage 2: Ripping {} files from {} folder(s)...",
            lines, selected_dirs
        ))))
        .unwrap();

    if cancelled() {
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

/// Traversing deeply nested directories can use a lot of memory.
///
/// For that reason we write the output to a file
pub fn traverse(
    dirs: Vec<PathBuf>,
    max_depth: u8,
    filter: impl Fn(&Path) -> bool,
    callback: impl Fn(u64),
) -> (BufReader<File>, u64) {
    // create a file in read-write mode
    // TODO: make this a temporary file, but I'll also need to figure out how to resume...
    // let mut file: File = OpenOptions::new()
    //     .read(true)
    //     .write(true)
    //     .create(true)
    //     .truncate(true)
    //     .open("./test.txt") // todo
    //     .unwrap();
    let mut file = tempfile::tempfile().expect("Creating a temporary file");

    // store the number of entries
    let mut lines: u64 = 0;

    // traverse list of directories, output to a file
    'traversal: for folder in dirs.into_iter() {
        for entry in WalkDir::new(folder).max_depth(max_depth as usize).into_iter() {
            if cancelled() {
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
    };

    // Rewind cursor to beginning
    file.rewind().unwrap();

    // Wrap file in bufreader and return
    (BufReader::new(file), lines)
}

pub type Batch<T> = Arc<Mutex<Vec<T>>>;

#[derive(Copy, Clone)]
struct NextBatch;

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
    pub fn new(
        file: &'io mut BufReader<File>,
        batch_size: usize,
        ripper: Arc<Ripper>,
        cfg: SampleRippingConfig,
        subscr_tx: AsyncSender<ThreadMsg>,
    ) -> Batcher<'io> {
        GLOBAL_TRACKER.set_batch_size(batch_size);

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
            const SUB_BATCH_SIZE: usize = 576;
            use rayon::prelude::*;

            let destination = cfg.destination;
            let self_contained = cfg.self_contained;

            GLOBAL_TRACKER.set_sub_batch_size(SUB_BATCH_SIZE);

            let rip_parallel = move |batch: &[String]| {
                batch.par_iter().for_each(|file| {
                    if cancelled() {
                        return;
                    }

                    let result = extract(&file, &destination, &ripper, self_contained);

                    // Send an update to the subscription
                    let _ = match result {
                        Ok(_) => subscr_tx.send(ThreadMsg::Progress(None)),

                        Err(error) => {
                            let error = Some(Failed::new(file.into(), error));
                            subscr_tx.send(ThreadMsg::Progress(error))
                        }
                    };
                });
            };

            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(cfg.worker_threads)
                .build()
                .unwrap();

            pool.spawn(move || {
                while let Ok(batch) = batch_rx.recv() {
                    for sub_batch in batch.lock().chunks(SUB_BATCH_SIZE) {
                        if cancelled() {
                            break;
                        }

                        rip_parallel(sub_batch);
                        GLOBAL_TRACKER.incr_sub_batch_number();
                    }

                    // Tell the batcher we're done so that it can send the next round
                    let _ = worker_tx.send(NextBatch);
                }
            });
        };

        batcher
    }

    pub fn start(&mut self) {
        let mut is_last_batch = false;

        while !self.state.complete && !cancelled() {
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
        // Aquire buffer
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
        GLOBAL_TRACKER.incr_batch_number();

        // Have a way of notifiying the caller that this is is the last batch,
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

            let Some(ext) = path
                .extension()
                .map(|f| f.to_str())
                .flatten()
            else {
                return false;
            };

            EXT.contains(&ext)
        },

        false => |_: &Path| true,
    }
}

#[derive(Default)]
struct State {
    pub complete: bool,
}

#[derive(Debug, Clone)]
pub struct Failed {
    pub path: PathBuf,
    pub reason: Box<str>,
    filename: Box<str>,
}

impl std::fmt::Display for Failed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed: {}, reason: {}",
            self.path.display(),
            &self.reason
        )
    }
}

impl Failed {
    pub fn new(path: String, error: xmodits_lib::interface::Error) -> Self {
        let path: PathBuf = path.into();
        Self {
            filename: file_name(&path).into(),
            path,
            reason: error.to_string().into(),
        }
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }
}

#[derive(Default, Clone, Copy)]
enum CurrentBatch {
    #[default]
    Batch1,
    Batch2,
}

impl CurrentBatch {
    pub fn next(self) -> Self {
        match self {
            Self::Batch1 => Self::Batch2,
            Self::Batch2 => Self::Batch1,
        }
    }

    pub fn switch(&mut self) {
        // println!("---------- SWITCHING ----------");
        *self = self.next();
    }
}

struct Buffer<T> {
    current: CurrentBatch,
    buf_1: Batch<T>,
    buf_2: Batch<T>,
}

impl<T> Buffer<T> {
    fn init(batch_size: usize) -> Self {
        Self {
            current: CurrentBatch::Batch1,
            buf_1: Self::alloc(batch_size),
            buf_2: Self::alloc(batch_size),
        }
    }

    fn current_buffer(&self) -> Batch<T> {
        match self.current {
            CurrentBatch::Batch1 => self.buf_1.clone(),
            CurrentBatch::Batch2 => self.buf_2.clone(),
        }
    }

    fn switch(&mut self) {
        self.current.switch()
    }

    fn alloc(batch_size: usize) -> Batch<T> {
        Arc::new(Mutex::new(Vec::with_capacity(batch_size)))
    }
}

// struct SubBatcher<'a, T> {
//     batch: &'a [T],
//     size: usize,
//     count: usize,
// }

// impl<'a, T> SubBatcher<'a, T> {
//     pub fn new(batch: &'a [T], size: usize) -> Self {
//         Self {
//             batch,
//             size,
//             count: 0,
//         }
//     }
// }

// impl<'a, T> Iterator for SubBatcher<'a, T> {
//     type Item = &'a [T];

//     fn next(&mut self) -> Option<Self::Item> {
//         let offset = self.size * self.count;
//         let len = self.batch.len();

//         // uses short circuiting so the order matters here
//         if len == 0 || offset > (len - 1) {
//             return None;
//         };

//         self.count += 1;
//         let end = offset + self.size;

//         match end > len {
//             true => self.batch.get(offset..),
//             false => self.batch.get(offset..end),
//         }
//     }
// }
