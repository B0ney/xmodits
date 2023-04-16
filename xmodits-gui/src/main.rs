#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // show logs when debugging
mod core;
mod gui;
mod simple;
use crate::core::panic_handler::panic::set_panic_hook;
use std::path::PathBuf;

fn main() {
    win_attach_terminal();
    set_panic_hook();
    init_logging();

    let args: Vec<PathBuf> = std::env::args()
        .skip(1)
        .map(|arg| PathBuf::new().join(arg))
        .collect();

    if args.is_empty() {
        return gui::App::start();
    }

    // Allow the user to drag abd drop modules onto the binary
    simple::rip(args);
}

fn init_logging() {
    use tracing::subscriber::set_global_default;
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
    // will be written to stdout.
    set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )
    .expect("setting default subscriber failed");
}

/// ``WINDOWS ONLY``: Have the application write to the terminal even with
/// ``[windows_subsystem = "windows"]``
///
/// This allows logs to be displayed when launched from the terminal.
fn win_attach_terminal() {
    #[cfg(windows)]
    unsafe {
        use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }
}
