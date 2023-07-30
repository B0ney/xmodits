#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // show logs when debugging
mod core;
mod gui;
mod simple;
use crate::core::panic_handler::panic::set_panic_hook;
use std::path::PathBuf;
mod logger;
pub mod font;
pub mod icon;

fn main() {
    logger::win_attach_terminal();
    set_panic_hook();
    logger::init_logging();

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
