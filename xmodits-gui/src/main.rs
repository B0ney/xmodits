// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // show logs when debugging
mod core;
// #[allow(unused)]
mod gui;
mod simple;
use crate::core::panic_handler::panic::set_panic_hook;
use std::{env, path::PathBuf};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
// use crate::core::dialog;

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

