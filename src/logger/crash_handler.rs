#[cfg(panic = "abort")]
compile_error!("Compiling XMODITS with `panic=\"abort\"` will make crash handling impossible, or mostly useless.");

use rand::Rng;
use tokio::sync::mpsc::{self, Sender};

use crate::screen::build_info;
use crate::{dialog, ripper::stop_flag};
use std::borrow::Cow;
use std::env;
use std::fmt::{Display, Write};
use std::fs::File;
use std::panic::{Location, PanicInfo};
use std::path::PathBuf;
use std::sync::OnceLock;

static PANIC_SIGNAL: OnceLock<Sender<Panic>> = OnceLock::new();

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Panic {
    pub saved_to: Option<PathBuf>,
    pub line: Option<u32>,
    pub file: String,
    pub message: String,
    pub backtrace: String,
    pub build_info: String,
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
                "Panic occurred in file:\n '{}'\nat line: {}.",
                location.file(),
                location.line(),
            )
            .into(),
            None => "Can't get location information...".into(),
        };

        write!(f, "{location}\n'{}'", self.message())
    }
}

/// Provide human friendly crash reporting
pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(move |panic_info| {
        stop_flag::set_abort();

        let dump = Dump::from_panic(panic_info);
        // todo: don't include backtrace for Eq impl
        let backtrace = std::backtrace::Backtrace::force_capture().to_string();
        // let backtrace = String::new();

        let build_info = build_info::info(true).fold(String::new(), |mut out, (label, value)| {
            writeln!(&mut out, "{label}: {value}").unwrap();
            out
        });

        let panic_log = Panic {
            saved_to: Some(PathBuf::new()),
            file: dump.file().to_owned(),
            line: dump.line(),
            message: dump.message().to_owned(),
            backtrace,
            build_info,
        };
        // TODO: save crash log to file
        // let temp_dir = std::env::temp_dir();
        // let filename = format!(
        //     "XMODITS-v{}-CRASH-{:X}",
        //     env!("CARGO_PKG_VERSION"),
        //     rand::thread_rng().gen::<u32>()
        // );

        if let Some(sender) = PANIC_SIGNAL.get() {
            let _ = sender.blocking_send(panic_log.clone());
        }

        tracing::error!(
            "FATAL ERROR: \n{}\n\nBACKTRACE:\n{}",
            &panic_log.message,
            &panic_log.backtrace
        );

        let message = dump.to_string();
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
pub fn subscription() -> iced::Subscription<Panic> {
    use iced::futures::SinkExt;
    use std::any::TypeId;

    struct PanicSignal;

    iced::subscription::channel(TypeId::of::<PanicSignal>(), 100, |mut output| async move {
        let (tx, mut rx) = mpsc::channel(32);

        PANIC_SIGNAL.set(tx).unwrap();

        loop {
            let msg = rx.recv().await.expect("sender");
            output.send(msg).await.expect("sending panic")
        }
    })
}
