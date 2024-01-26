//! Keep track of modules that might have caused the program to panic

use iced::futures::SinkExt;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::any::TypeId;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;

pub(crate) static BAD_MODULES: Lazy<BadModules> = Lazy::new(BadModules::default);

#[derive(Default)]
pub(crate) struct BadModules {
    modules: RwLock<Vec<PathBuf>>,
    callbacks: RwLock<Vec<Box<dyn Fn(&Path) + Send + Sync + 'static>>>,
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
        self.callbacks
            .read()
            .iter()
            .for_each(|callback| callback(&path));

        self.modules.write().push(path);
    }

    pub fn copy(&self) -> Vec<PathBuf> {
        self.modules.read().clone()
    }

    pub fn total(&self) -> usize {
        self.modules.read().len() as usize
    }
}

/// Adds the given path to the global "BAD_MODULES" if the calling function panics.
///
/// This won't work if the panicking strategy is "abort".
pub struct LogOnPanic<'a> {
    suspect_file: &'a Path,
}

impl<'a> LogOnPanic<'a> {
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

impl<'a> Drop for LogOnPanic<'a> {
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
        let (tx, mut rx) = mpsc::channel(100);

        BAD_MODULES.register_callback(move |path| {
            let path = path.to_owned();
            let _ = tx.blocking_send(Added { path });
        });

        loop {
            let added = rx.recv().await.expect("receiving from BAD_MODULES");
            let _ = output.send(added).await;
        }
    })
}
