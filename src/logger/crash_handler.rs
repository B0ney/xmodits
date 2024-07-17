#[cfg(panic = "abort")]
compile_error!("Compiling XMODITS with `panic=\"abort\"` will make crash handling impossible, or mostly useless.");

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rand::Rng;
use tokio::sync::mpsc::{self, Sender};

use crate::{dialog, ripper::stop_flag};
use std::any::Any;
use std::borrow::Cow;
use std::collections::HashSet;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io::Write as _;
use std::panic::{Location, PanicInfo};
use std::path::PathBuf;
use std::sync::OnceLock;

static PANIC_SIGNAL: OnceLock<Sender<SavedPanic>> = OnceLock::new();

// Prevent panic handler from creating duplicate error logs
static PANICS: Lazy<Mutex<HashSet<Panic>>> = Lazy::new(|| Mutex::new(HashSet::new()));

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SavedPanic {
    pub panic: Panic,
    pub saved_to: Option<PathBuf>,
}

impl SavedPanic {
    pub fn line(&self) -> Option<u32> {
        self.panic.line
    }

    pub fn message(&self) -> &str {
        &self.panic.message
    }

    pub fn file(&self) -> &str {
        &self.panic.file
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Panic {
    pub line: Option<u32>,
    pub file: String,
    pub message: String,
    #[cfg(feature = "backtrace")]
    pub backtrace: String,
    #[cfg(feature = "built")]
    pub build_info: String,
}

impl PartialEq for Panic {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.file == other.file && self.message == other.message
    }
}

impl Hash for Panic {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.line.hash(state);
        self.file.hash(state);
        self.message.hash(state);
    }
}

#[derive(Default, Debug)]
struct Dump<'a> {
    pub location: Option<&'a Location<'a>>,
    pub message: Option<Cow<'a, str>>,
}

impl<'a> Dump<'a> {
    fn file(&self) -> &str {
        match self.location {
            Some(file) => file.file(),
            None => "",
        }
    }
}

impl<'a> Dump<'a> {
    fn from_panic(panic_info: &'a PanicInfo) -> Self {
        let location = panic_info.location();

        let message: Option<Cow<str>> = match panic_info.payload().downcast_ref::<String>() {
            Some(e) => Some(e.into()),
            None => panic_info
                .payload()
                .downcast_ref::<&'static str>()
                .map(|s| Cow::Borrowed(*s)),
        };

        Self { location, message }
    }

    fn message(&self) -> &str {
        match &self.message {
            Some(msg) => msg,
            None => "Unknown Panic",
        }
    }

    fn line(&self) -> Option<u32> {
        self.location.map(|f| f.line())
    }
}

impl<'a> Display for Dump<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location: Cow<str> = match self.location {
            Some(location) => format!(
                "Panic occurred in file:\n'{}'\nat line: {}.",
                location.file(),
                location.line(),
            )
            .into(),
            None => "Can't get location information...".into(),
        };

        write!(f, "{location}\n\n'{}'", self.message())
    }
}

/// Provide human friendly crash reporting
pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(move |panic_info| {
        // Set stop_flag to abort to stop ripping process
        stop_flag::set_abort();

        // gather necessary crash information
        let dump = Dump::from_panic(panic_info);

        #[cfg(feature = "backtrace")]
        let backtrace = std::backtrace::Backtrace::force_capture().to_string();

        #[cfg(feature = "built")]
        let build_info = {
            use crate::screen::build_info::info;
            use std::fmt::Write;

            info(true).fold(String::new(), |mut out, (label, value)| {
                writeln!(&mut out, "{label}: {value}").unwrap();
                out
            })
        };

        let panic_log = Panic {
            file: dump.file().to_owned(),
            line: dump.line(),
            message: dump.message().to_owned(),
            #[cfg(feature = "backtrace")]
            backtrace,
            #[cfg(feature = "built")]
            build_info,
        };

        // If another thread wrote the same panic, early exit.
        // We don't need duplicate crash logs and message boxes...
        if !PANICS.lock().insert(panic_log.clone()) {
            tracing::warn!("skipping duplicate panic");
            return;
        }

        #[cfg(feature = "backtrace")]
        tracing::error!(
            "FATAL ERROR: \n{}\n\nBACKTRACE:\n{}",
            &panic_log.message,
            &panic_log.backtrace
        );

        #[cfg(not(feature = "backtrace"))]
        tracing::error!("FATAL ERROR: \n{}", &panic_log.message,);

        // Save crash log to user's downloads folder
        let saved_to = {
            let filename = format!(
                "XMODITS-v{}-CRASH-{:X}.txt",
                env!("CARGO_PKG_VERSION"),
                rand::thread_rng().gen::<u16>()
            );

            let log_path = dirs::download_dir().unwrap_or_default().join(filename);

            let write_error = |mut file: File| {
                let _ = write!(&mut file, "XMODITS CRASH LOG\n\n");

                #[cfg(feature = "built")]
                let _ = write!(
                    &mut file,
                    "APPLICATION BUILD INFO:\n{}\n",
                    &panic_log.build_info
                );

                let _ = write!(
                    &mut file,
                    "SOURCE: {} \n\n\
                    LINE: {} \n\n\
                    MESSAGE: {} \n\n\
                    ",
                    dump.file(),
                    if let Some(line) = dump.line() {
                        format!("{line}")
                    } else {
                        "Unknown".to_owned()
                    },
                    dump.message(),
                );

                #[cfg(feature = "backtrace")]
                let _ = write!(
                    &mut file,
                    "BACKTRACE: \n\
                    {}",
                    &panic_log.backtrace
                );
            };

            match std::fs::File::create(&log_path) {
                Err(_) => None,
                Ok(file) => {
                    write_error(file);
                    Some(log_path)
                }
            }
        };

        let message = dump.to_string();
        let message = match &saved_to {
            Some(location) => format!(
                "{}\n\nA crash log was written to: {}",
                message,
                location.display()
            ),
            None => message,
        };

        // Send a copy of the panic + location of where it's saved
        if let Some(sender) = PANIC_SIGNAL.get() {
            let _ = sender.blocking_send(SavedPanic {
                panic: panic_log.clone(),
                saved_to,
            });
        }

        let message = move || dialog::critical_error(&message);

        let msg_box = std::thread::spawn(message);

        // Only block if panic came from main thread.
        // This allows other threads to unwind without
        // having the user to close the dialog box.
        if let Some("main") = std::thread::current().name() {
            msg_box.join().unwrap();
        }
    }));
}

/// Emits events when a panic occurs
pub fn subscription() -> iced::Subscription<SavedPanic> {
    use iced::futures::SinkExt;
    use std::any::TypeId;

    struct PanicSignal;

    iced::Subscription::run_with_id(
        PanicSignal.type_id(),
        iced::stream::channel(100, |mut output| async move {
            let (tx, mut rx) = mpsc::channel(32);
            PANIC_SIGNAL.set(tx).unwrap();

            loop {
                let msg = rx.recv().await.expect("sender");
                output.send(msg).await.expect("sending panic")
            }
        }),
    )
}
