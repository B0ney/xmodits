use rand::Rng;
use tokio::sync::mpsc::{self, Sender};

use crate::screen::build_info;
use crate::{dialog, ripper::stop_flag};
use std::borrow::Cow;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::panic::{Location, PanicInfo};
use std::path::PathBuf;
use std::sync::OnceLock;

static PANIC_SIGNAL: OnceLock<Sender<Panic>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Panic {
    // file: PathBuf,
    // line: usize,
    pub message: String,
    pub backtrace: String,
}

#[derive(Default, Debug)]
struct Dump<'a> {
    pub location: Option<&'a Location<'a>>,
    pub message: Option<Cow<'a, str>>,
}

impl <'a>Dump<'a> {
    fn file(&self) -> &str {
        match self.location {
            Some(file) => file.file(),
            None => ""
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
}

impl<'a> Display for Dump<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: &str = match &self.message {
            Some(msg) => &msg,
            None => "Panic occurred.",
        };

        let location: Cow<str> = match self.location {
            Some(location) => format!(
                "Panic occurred in file:\n '{}'\nat line: {}.",
                location.file(),
                location.line(),
            )
            .into(),
            None => "Can't get location information...".into(),
        };

        write!(f, "{location}\n'{message}'")
    }
}

/// Provide human friendly crash reporting
pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(move |panic_info| {
        stop_flag::set_abort();

        let message = Dump::from_panic(panic_info).to_string();
        let backtrace = std::backtrace::Backtrace::force_capture().to_string();
        let build_info = build_info::info(true);


        // TODO: save crash log to file
        // let temp_dir = std::env::temp_dir();
        // let filename = format!(
        //     "XMODITS-v{}-CRASH-{:X}",
        //     env!("CARGO_PKG_VERSION"),
        //     rand::thread_rng().gen::<u32>()
        // );

        if let Some(sender) = PANIC_SIGNAL.get() {
            let _ = sender.blocking_send(Panic {
                message: message.clone(),
                backtrace: backtrace.clone(),
            });
        }

        tracing::error!("FATAL ERROR: \n{}\n\nBACKTRACE:\n{}", message, backtrace);

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
