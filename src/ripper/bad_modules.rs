//! Keep track of modules that might have caused the program to panic

use iced::futures::SinkExt;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::any::TypeId;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::mpsc::{self, Receiver};

pub static BAD_MODULES: Lazy<BadModules> = Lazy::new(BadModules::default);

#[derive(Default)]
pub struct BadModules {
    modules: RwLock<Vec<PathBuf>>,
    callbacks: RwLock<Vec<Box<dyn Fn(&Path) + Send + Sync + 'static>>>,
    total: AtomicU64,
}

impl BadModules {
    pub fn register_callback<F>(&self, callback: F)
    where
        F: Fn(&Path) + Send + Sync + 'static,
    {
        self.callbacks.write().push(Box::new(callback));
        tracing::info!("Registered callback!");
    }

    fn push(&self, path: PathBuf) {
        // Adding modules should happen *before* we fetch the total.
        let _ = self.total.fetch_add(1, Ordering::Release);

        self.callbacks
            .read()
            .iter()
            .for_each(|callback| callback(&path));

        self.modules.write().push(path);
    }

    pub fn copy(&self) -> Vec<PathBuf> {
        self.modules.read().clone()
    }

    pub fn total(&self) -> u64 {
        self.total.load(Ordering::Acquire)
    }
}

pub struct RipperPanic<'a> {
    suspect_file: &'a Path,
}

impl<'a> RipperPanic<'a> {
    pub fn new(suspect_file: &'a Path) -> Self {
        Self { suspect_file }
    }

    pub fn execute<T, F>(self, func: F) -> T
    where
        F: Fn(&'a Path) -> T,
    {
        func(self.suspect_file)
    }
}

impl<'a> Drop for RipperPanic<'a> {
    fn drop(&mut self) {
        #[cold]
        fn add(path: &Path) {
            BAD_MODULES.push(path.to_owned());
        }

        if std::thread::panicking() {
            add(self.suspect_file);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Added {
    pub path: PathBuf,
}

/// This subscription reports when a module caused a crash
pub fn subscription() -> iced::Subscription<Added> {
    iced::subscription::channel(TypeId::of::<BadModules>(), 100, |mut output| async move {
        let mut state = None::<Receiver<Added>>;

        loop {
            match &mut state {
                None => {
                    let (tx, rx) = mpsc::channel(100);

                    BAD_MODULES.register_callback(move |path| {
                        let _ = tx.blocking_send(Added {
                            path: path.to_owned(),
                        });
                    });

                    state = Some(rx);
                }
                Some(rx) => {
                    if let Some(msg) = rx.recv().await {
                        let _ = output.send(msg).await;
                    }
                }
            }
        }
    })
}
