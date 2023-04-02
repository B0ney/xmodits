use parking_lot::Mutex;
use std::borrow::Cow;
use std::path::Path;
use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Sender, Receiver};
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
    BufReader::new(file)
        .lines()
        .filter_map(|f| f.ok())
        .for_each(|f| println!("{}", f));
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
}

impl <'io>Batcher<'io> {
    pub fn new(
        file: &'io mut BufReader<File>, 
        batch_size: usize, 
        batch_number: usize
    ) -> Batcher<'io> {
        let (tx, rx) = mpsc::channel::<Batch<String>>();

        let mut batcher = Self {
            file,
            batch_size,
            batch_number,
            state: State::default(),
            buf_1: Self::alloc(batch_size),
            buf_2: Self::alloc(batch_size),
            sender: tx,
        };

        batcher.load(CurrentBatch::Batch1);
        spawn_worker_thread(rx);

        batcher
    }

    pub fn resume(
        file: &'io mut BufReader<File>, 
        batch_size: usize, 
        batch_number: usize
    ) -> Self {
        let _ = file.lines().nth(batch_number * batch_size);
        Self::new(file, batch_size, batch_number)
    }

    pub fn start(&mut self) {
        self.sender.send(self.get_batch_current()).unwrap();
        self.load_next_batch();
    }

    pub fn get_batch_current(&self) -> Batch<String> {
        match self.state.batch {
            CurrentBatch::Batch1 => self.buf_1.clone(),
            CurrentBatch::Batch2 => self.buf_2.clone(),
        }
    }

    pub fn load_next_batch(&mut self) {
        self.state.batch.switch();
        self.load(self.state.batch);
    }

    pub fn load(&mut self, batch: CurrentBatch) {
        // Aquire buffer, clear it
        let mut buffer = match batch {
            CurrentBatch::Batch1 => self.buf_1.lock(),
            CurrentBatch::Batch2 => self.buf_2.lock(),
        };

        buffer.clear();

        self.file
            .lines()
            .take(self.batch_size)
            .filter_map(|f| f.ok())
            .for_each(|line| buffer.push(line));

        // If the buffer size is less than the defined batch size,
        // we're done
        if buffer.len() < self.batch_size {}

        self.batch_number += 1;
    }
    
    pub fn alloc(batch_size: usize) -> Batch<String> {
        Arc::new(Mutex::new(Vec::with_capacity(batch_size)))
    }
}

fn spawn_worker_thread(rx: Receiver<Batch<String>>) {

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
        *self = self.next();
    }
}


#[test]
fn traverse_() {
    traverse(vec!["~/Downloads/".into()]);
    // load();
}
