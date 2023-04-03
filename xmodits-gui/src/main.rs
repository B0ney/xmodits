// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // show logs when debugging
mod core;
// #[allow(unused)]
mod gui;
mod simple;
use std::{env, path::PathBuf};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use crate::core::dialog;

fn main() {
    set_panic_hook();
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        .finish(); // completes the builder.

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args: Vec<PathBuf> = env::args()
        .skip(1)
        .map(|arg| PathBuf::new().join(arg))
        .collect();

    // The user may want to just drag and drop a module without any thought
    if args.is_empty() {
        info!("Starting gui");
        gui::App::start();
    } else {
        simple::rip(args);
    }
}

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        // let backtrace = std::backtrace::Backtrace::force_capture();
        // println!("{}", backtrace.to_string());
        // std::thread::park();
        let info = match panic_info.location() {
            Some(location) => format!(
                "Panic occurred in file '{}' at line {}",
                location.file(),
                location.line()
            ),
            None => String::from("Panic occurred but can't get location information..."),
        };

        let message = match (
            panic_info.payload().downcast_ref::<String>(),
            panic_info.payload().downcast_ref::<&str>(),
        ) {
            (Some(e), None) => e.as_str(),
            (None, Some(e)) => e,
            _ => "Panic occured",
        };

        dialog::critical_error(&format!("{}\n{:?}", info, message));

        std::process::exit(1)
    }));
}
