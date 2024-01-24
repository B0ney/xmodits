use rand::Rng;

use crate::ripper::extraction::error_handler::random_name;
use crate::screen::build_info;
use crate::{dialog, ripper::stop_flag};
use std::borrow::Cow;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::panic::{Location, PanicInfo};
use std::path::PathBuf;

#[derive(Default, Debug)]
struct Dump<'a> {
    pub location: Option<&'a Location<'a>>,
    pub message: Option<Cow<'a, str>>,
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
    std::panic::set_hook(Box::new(|panic_info| {
        stop_flag::set_flag(stop_flag::StopFlag::Abort);
        let message = Dump::from_panic(panic_info).to_string();
        let backtrace = std::backtrace::Backtrace::force_capture().to_string();
        let build_info = build_info::info(true);

        tracing::error!("FATAL ERROR: \n{}\n\nBACKTRACE:\n{}", message, backtrace);

        // Spawn thread to ensure that it can't block or be blocked by the application
        // TODO: is this bad?
        std::thread::spawn(move || {
            dialog::critical_error(&message);

            // TODO: save crash log to file
            
            // let temp_dir = std::env::temp_dir();
            // let filename = format!(
            //     "XMODITS-v{}-CRASH-{:X}",
            //     env!("CARGO_PKG_VERSION"),
            //     rand::thread_rng().gen::<u32>()
            // );

            // TODO: get module(s) that might have caused a crash
            if let Some(files) = crate::ripper::extraction::CURSED_MODULES.lock().first().take() {
                tracing::error!("Problematic module: {}", files.display());
                dialog::critical_error(&files.display().to_string());
            }

            std::process::exit(1)
        });
    }));
}
