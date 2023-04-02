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

use walkdir::WalkDir;

/// Traversing deeply nested directories can use a lot of memory.
///
/// For that reason we write the output to a file
///
/// Todo: make this async?
/// Todo: expand tilde
pub fn traverse(dirs: Vec<String>) {
    // create a file in read-write mode
    let mut file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./test.txt")
        .unwrap();

    // traverse list of directories, output to a file
    let max_depth = 5;
    dirs.into_iter().for_each(|f| {
        WalkDir::new(shellexpand::tilde(&f).as_ref())
            .max_depth(max_depth)
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| f.path().is_file())
            .for_each(|f| {
                file.write_fmt(format_args!("{}\n", f.path().display()))
                    .unwrap()
            })
    });

    // rewind cursor to beginning
    file.rewind().unwrap();

    // wrap file to bufreader
    let mut file = BufReader::new(file);

    let mut batcher = Batcher::new(&mut file, 2048);
    batcher.start();
    dbg!(batcher.batch_number);
    // batcher.handle.unwrap().join();
}

pub type Batch<T> = Arc<Mutex<Vec<T>>>;

struct Batcher<'io> {
    file: &'io mut BufReader<File>,
    batch_size: usize,
    batch_number: usize,
    state: State,
    buf_1: Batch<String>,
    buf_2: Batch<String>,
    sender: Sender<Batch<String>>,
    recv: Receiver<Msg>,
    pub handle: Option<std::thread::JoinHandle<()>>,
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
            handle: None,
        };

        batcher.load();
        batcher.handle = Some(spawn_worker_thread(rx, w_tx));

        batcher
    }

    // pub fn resume(file: &'io mut BufReader<File>, batch_size: usize, batch_number: usize) -> Self {
    //     let _ = file.lines().nth(batch_number * batch_size);
    //     Self::new(file, batch_size, batch_number)
    // }

    pub fn start(&mut self) {
        let mut is_last = false;

        while !self.state.complete {
            // If this is the last batch, set the state to complete 
            // and send the last batch. When complete this loop terminates.
            if is_last {
                self.state.complete = true;
            }

            // Send the current batch to the worker thread
            self.sender.send(self.get_batch_current()).unwrap();

            // While the worker thread is dealing with the first batch,
            // prepare the next batch. 
            is_last = self.load_next_batch();

            // wait for the worker to finish, then loop
            match self.recv.recv() {
                Ok(msg) => match msg {
                    Msg::Next => continue,
                },
                Err(_) => {
                    dbg!(":(");
                    break
                },
            }
        }
    }

    pub fn get_batch_current(&self) -> Batch<String> {
        match self.state.batch {
            CurrentBatch::Batch1 => self.buf_1.clone(),
            CurrentBatch::Batch2 => self.buf_2.clone(),
        }
    }

    pub fn load_next_batch(&mut self) -> bool {
        self.state.batch.switch();
        self.load()
    }

    pub fn load(&mut self)  -> bool {
        let mut is_last = false;

        // Aquire buffer
        let buffer = self.get_batch_current();
        let mut buffer = buffer.lock();

        // Clear it
        buffer.clear();

        self.file
            .lines()
            .take(self.batch_size)
            .filter_map(|f| f.ok())
            .for_each(|line| buffer.push(line));

        // If the buffer size is less than the defined batch size then we're done
        if buffer.len() < self.batch_size {
            is_last = true
        }

        self.batch_number += 1;
        is_last
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
    rayon::ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();

    std::thread::spawn(move || loop {
        match rx.recv() {
            Ok(batch) => {
                batch.lock().par_iter().enumerate().for_each(|(idx, f)| {
                    // do something expensive
                    // println!("{} - {}", idx, f);
                    std::thread::sleep(std::time::Duration::from_millis(2));
                });
                tx.send(Msg::Next).unwrap();
            }
            Err(_) => {
                dbg!("channel closed");
                break
            },
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
    pub fn current(self) -> Self {
        self
    }

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
    traverse(vec!["~/Downloads/".into()]);
    // load();
}
