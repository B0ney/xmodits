//! Keep track of modules that might have caused the program to panic

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::{
    any::Any,
    path::{Path, PathBuf},
};
use tokio::sync::mpsc;

pub(crate) static BAD_MODULES: Lazy<BadModules> = Lazy::new(BadModules::default);

type Subscriber = Box<dyn Fn(PathBuf) + Send + Sync + 'static>;

#[derive(Default)]
pub(crate) struct BadModules {
    subscribers: RwLock<Vec<Subscriber>>,
}

impl BadModules {
    pub(crate) fn add_subscriber<F>(&self, subscriber: F)
    where
        F: Fn(PathBuf) + Send + Sync + 'static,
    {
        self.subscribers.write().push(Box::new(subscriber));
        tracing::info!("Registered callback!");
    }

    fn push(&self, path: PathBuf) {
        self.subscribers
            .read()
            .iter()
            .for_each(|notify| notify(path.clone()));
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

    iced::Subscription::run_with_id(
        BAD_MODULES.type_id(),
        iced::stream::channel(32, |mut output| async move {
            let (tx, mut rx) = mpsc::channel(32);

            BAD_MODULES.add_subscriber(move |path| {
                let _ = tx.blocking_send(path);
            });

            loop {
                let added = rx.recv().await.unwrap();
                let _ = output.send(added).await;
            }
        }),
    )
}
