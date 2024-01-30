//! Keep track of modules that might have caused the program to panic

use once_cell::sync::Lazy;
use parking_lot::RwLock;
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
}

/// Helper function to add the given path to the global BAD_MODULES if the calling closure panics.
/// Doesn't work if panic strategy isn't unwind.
#[inline]
pub fn log_file_on_panic<'a, T, F>(path: &'a Path, func: F) -> T
where
    F: Fn(&'a Path) -> T,
{
    struct LogOnUnwind<'a> {
        suspect_file: &'a Path,
    }

    impl<'a> LogOnUnwind<'a> {
        fn new(suspect_file: &'a Path) -> Self {
            Self { suspect_file }
        }

        fn execute<T, F>(self, func: F) -> T
        where
            F: Fn(&'a Path) -> T,
        {
            func(self.suspect_file)
        }
    }

    impl<'a> Drop for LogOnUnwind<'a> {
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

    LogOnUnwind::new(path).execute(func)
}

/// This subscription reports when a module caused a crash
pub fn subscription() -> iced::Subscription<PathBuf> {
    use iced::futures::SinkExt;
    use std::any::TypeId;

    iced::subscription::channel(TypeId::of::<BadModules>(), 100, |mut output| async move {
        let (tx, mut rx) = mpsc::channel(100);

        BAD_MODULES.register_callback(move |path| {
            let path = path.to_owned();
            let _ = tx.blocking_send(path);
        });

        loop {
            let added = rx.recv().await.expect("receiving from BAD_MODULES");
            let _ = output.send(added).await;
        }
    })
}
