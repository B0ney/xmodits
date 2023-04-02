use crate::core::cfg::{SampleNameConfig, SampleRippingConfig};
use parking_lot::Mutex;
use std::borrow::Cow;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, Write},
    path::PathBuf,
};
use xmodits_lib::interface::ripper::Ripper;

use walkdir::WalkDir;

pub fn rip(paths: Vec<PathBuf>, cfg: SampleRippingConfig) {
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

    let max_depth = match cfg.folder_recursion_depth {
        0 => 1,
        d => d,
    };

    let ripper = Arc::new(Ripper::new(
        cfg.naming.build_func(),
        cfg.exported_format.into(),
    ));

    stage_1(files);
    stage_2(folders);
}

fn stage_1(files: Vec<PathBuf>) {}
/// todo add documentation
fn stage_2(folders: Vec<PathBuf>) {}

fn a(dirs: Vec<PathBuf>) {
    let mut file = traverse(dirs, 7, |_| true);
    let mut batcher = Batcher::new(&mut file, 2048);
    batcher.start();
    dbg!(batcher.batch_number);
    // batcher.handle.unwrap().join();
}

/// Traversing deeply nested directories can use a lot of memory.
///
/// For that reason we write the output to a file
///
/// Todo: make this async?
pub fn traverse(
    dirs: Vec<PathBuf>,
    max_depth: u8,
    filter: impl Fn(&Path) -> bool,
) -> BufReader<File> {
    // create a file in read-write mode
    let mut file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./test.txt")
        .unwrap();

    // traverse list of directories, output to a file
    dirs.into_iter().for_each(|f| {
        WalkDir::new(f)
            .max_depth(max_depth as usize)
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| f.path().is_file() && filter(f.path()))
            .for_each(|f| {
                file.write_fmt(format_args!("{}\n", f.path().display()))
                    .unwrap()
            })
    });

    // Rewind cursor to beginning
    file.rewind().unwrap();

    // Wrap file in bufreader
    BufReader::new(file)
}

pub type Batch<T> = Arc<Mutex<Vec<T>>>;

///
///
struct Batcher<'io> {
    file: &'io mut BufReader<File>,
    batch_size: usize,
    batch_number: usize,
    state: State,
    buf_1: Batch<String>,
    buf_2: Batch<String>,
    sender: Sender<Batch<String>>,
    recv: Receiver<Msg>,
    // pub handle: Option<std::thread::JoinHandle<()>>,
}

enum Msg {
    Next,
    // Done,
}

impl<'io> Batcher<'io> {
    pub fn new(file: &'io mut BufReader<File>, batch_size: usize) -> Batcher<'io> {
        let (tx, rx) = mpsc::channel::<Batch<String>>();
        let (w_tx, w_rx) = mpsc::channel::<Msg>();

        let mut batcher = Self {
            file,
            batch_size,
            batch_number: 0,
            state: State::default(),
            buf_1: Self::alloc(batch_size),
            buf_2: Self::alloc(batch_size),
            sender: tx,
            recv: w_rx,
            // handle: None,
        };

        batcher.load();
        // batcher.handle = Some(spawn_worker_thread(rx, w_tx));
        spawn_worker_thread(rx, w_tx);

        batcher
    }

    pub fn resume(file: &'io mut BufReader<File>, batch_size: usize, batch_number: usize) -> Self {
        let _ = file.lines().nth(batch_number * batch_size);

        Self::new(file, batch_size)
    }

    pub fn start(&mut self) {
        let mut is_last_batch = false;

        while !self.state.complete {
            // If this is the last batch, set the state to complete
            // and send the last batch. When complete this loop terminates.
            if is_last_batch {
                self.state.complete = true;
            }

            // Send the current batch to the worker thread
            self.sender.send(self.get_current_batch()).unwrap();

            // While the worker thread is dealing with the first batch,
            // prepare the next batch. Ping-pong buffering ftw.
            is_last_batch = self.load_next_batch();

            // wait for the worker to finish, then loop
            match self.recv.recv() {
                Ok(msg) => match msg {
                    Msg::Next => continue,
                },
                Err(_) => break, // TODO
            }
        }
    }

    pub fn get_current_batch(&self) -> Batch<String> {
        match self.state.batch {
            CurrentBatch::Batch1 => self.buf_1.clone(),
            CurrentBatch::Batch2 => self.buf_2.clone(),
        }
    }

    /// Load the next batch of lines
    pub fn load_next_batch(&mut self) -> bool {
        self.state.batch.switch();
        self.load()
    }

    /// Store ``batch_size`` lines to the current buffer
    ///
    /// Returns true if the buffer is less than the defined batch size.
    ///
    /// This indicates we've reached the end of the file.
    pub fn load(&mut self) -> bool {
        // Aquire buffer
        let buffer = self.get_current_batch();
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

    pub fn alloc(batch_size: usize) -> Batch<String> {
        Arc::new(Mutex::new(Vec::with_capacity(batch_size)))
    }
}

fn spawn_worker_thread(
    rx: Receiver<Batch<String>>,
    tx: Sender<Msg>,
) -> std::thread::JoinHandle<()> {
    use rayon::prelude::*;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build()
        .unwrap();

    std::thread::spawn(move || loop {
        match rx.recv() {
            Ok(batch) => {
                pool.install(|| {
                    batch.lock().par_iter().enumerate().for_each(|(idx, f)| {
                        // do something expensive
                        // println!("{} - {}", idx, f);
                        std::thread::sleep(std::time::Duration::from_millis(2));
                    });
                });

                tx.send(Msg::Next).unwrap();
            }

            Err(_) => break,
        }
    })
}

#[derive(Default)]
struct State {
    pub complete: bool,
    pub batch: CurrentBatch,
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
        println!("---------- SWITCHING ----------");
        *self = self.next();
    }
}

#[test]
fn traverse_() {
    // cargo test --package xmodits-gui -- core::extraction::traverse_ --exact --nocapture
    a(vec!["~/Downloads".into()]);
    // panic!()
    // load();
}
