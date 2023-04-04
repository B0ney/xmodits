use crate::core::cfg::{SampleNameConfig, SampleRippingConfig};
use parking_lot::Mutex;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, Write},
    path::PathBuf,
};
use xmodits_lib::common::extract;
use xmodits_lib::interface::ripper::Ripper;

use walkdir::WalkDir;

#[derive(Debug)]
pub enum ThreadMsg {
    SetTotal(usize),
    Info(Option<String>),
    Progress(Option<Failed>),
    Done,
}

impl ThreadMsg {
    pub fn info(str: &str) -> Self {
        Self::Info(Some(str.to_owned()))
    }
}

use tokio::sync::mpsc::UnboundedSender as AsyncSender;

pub fn rip(tx: AsyncSender<ThreadMsg>, paths: Vec<PathBuf>, mut cfg: SampleRippingConfig) {
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

    stage_1(tx.clone(), files, ripper.clone(), &cfg);
    stage_2(tx.clone(), folders, ripper.clone(), cfg);

    tx.send(ThreadMsg::Done).unwrap();
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
    subscr_tx.send(ThreadMsg::SetTotal(files.len())).unwrap();

    subscr_tx
        .send(ThreadMsg::Info(Some(format!(
            "Stage 1: Ripping {} files...",
            files.len()
        ))))
        .unwrap();

    files.into_iter().for_each(|file| {
        let e = extract(file, &cfg.destination, ripper.as_ref(), cfg.self_contained);
        subscr_tx.send(ThreadMsg::Progress(None)).unwrap();
    });
}

/// todo add documentation
///
fn stage_2(
    subscr_tx: AsyncSender<ThreadMsg>,
    folders: Vec<PathBuf>,
    ripper: Arc<Ripper>,
    cfg: SampleRippingConfig,
) {
    if folders.is_empty() {
        return;
    }
    let selected_dirs = folders.len();
    subscr_tx
        .send(ThreadMsg::info("Traversing Directories..."))
        .unwrap();

    let (mut file, lines) = traverse(folders, cfg.folder_max_depth);
    subscr_tx.send(ThreadMsg::SetTotal(lines)).unwrap();

    subscr_tx
        .send(ThreadMsg::Info(Some(format!(
            "Stage 2: Ripping {} file from {} folder(s)...",
            lines, selected_dirs
        ))))
        .unwrap();

    let mut batcher = Batcher::new(&mut file, batch_size(lines), ripper, cfg, subscr_tx.clone());
    batcher.start();
}

fn batch_size(lines: usize) -> usize {
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
///
/// Todo: make this async?
pub fn traverse(
    dirs: Vec<PathBuf>,
    max_depth: u8,
    // filter: impl Fn(&Path) -> bool,
) -> (BufReader<File>, usize) {
    // create a file in read-write mode
    let mut file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./test.txt")
        .unwrap();

    // store the number of entries
    let mut lines: usize = 0;

    // traverse list of directories, output to a file
    dirs.into_iter().for_each(|f| {
        WalkDir::new(f)
            .max_depth(max_depth as usize)
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| f.path().is_file())
            .for_each(|f| {
                lines += 1;
                file.write_fmt(format_args!("{}\n", f.path().display()))
                    .unwrap()
            })
    });

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
    // pub handle: Option<std::thread::JoinHandle<()>>,
}

impl<'io> Batcher<'io> {
    pub fn new(
        file: &'io mut BufReader<File>,
        batch_size: usize,
        ripper: Arc<Ripper>,
        cfg: SampleRippingConfig,
        subscr_tx: AsyncSender<ThreadMsg>,
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
            // handle: None,
        };

        // load first buffer
        batcher.load();

        spawn_workers(
            batch_rx,
            worker_tx,
            subscr_tx,
            ripper,
            cfg.destination,
            cfg.self_contained,
        );

        batcher
    }

    // pub fn resume(file: &'io mut BufReader<File>, batch_size: usize, batch_number: usize) -> Self {
    //     let _ = file.lines().nth(batch_number * batch_size);

    //     Self::new(file, batch_size)
    // }

    pub fn start(&mut self) {
        let mut is_last_batch = false;

        while !self.state.complete {
            // If this is the last batch, set the state to complete
            // and send the last batch. When complete this loop terminates.
            if is_last_batch {
                self.state.complete = true;
            }

            // Send the current batch to the worker thread
            self.batch_tx.send(self.buffer.current_buffer()).unwrap();

            // While the worker thread is dealing with the first batch,
            // prepare the next batch. Ping-pong buffering ftw.
            is_last_batch = self.load_next_batch();

            // wait for the worker to finish, then loop
            match self.worker_rx.recv() {
                Ok(_) => continue,
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

        // Have a way of notifiying the caller that this is is the last batch,
        // and should not be called again.
        //
        // TODO: improve ergonomics of this function
        buffer.len() < self.batch_size
    }
}

fn spawn_workers(
    batch_rx: Receiver<Batch<String>>,
    worker_tx: Sender<NextBatch>,
    subscr_tx: AsyncSender<ThreadMsg>,
    ripper: Arc<Ripper>,
    destination: PathBuf,
    self_contained: bool,
) {
    use rayon::prelude::*;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(0)
        .build()
        .unwrap();

    pool.spawn(move || loop {
        match batch_rx.recv() {
            Ok(batch) => {
                batch.lock().par_iter().for_each(|file| {
                    let progress = ThreadMsg::Progress(
                        match extract(&file, &destination, ripper.as_ref(), self_contained) {
                            Ok(()) => None,
                            Err(error) => Some(Failed::new(file.into(), error)),
                        },
                    );

                    // Send and update to the subscription
                    subscr_tx.send(progress).unwrap()
                });

                // Tell the batcher we're done so that it can send the next round
                worker_tx.send(NextBatch).unwrap();
            }

            Err(_) => break,
        }
    });
}

#[derive(Default)]
struct State {
    pub complete: bool,
}

#[derive(Debug, Clone)]
pub struct Failed {
    path: PathBuf,
    reason: Box<str>,
}

impl Failed {
    pub fn new(path: String, error: xmodits_lib::interface::Error) -> Self {
        Self {
            path: path.into(),
            reason: error.to_string().into(),
        }
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
