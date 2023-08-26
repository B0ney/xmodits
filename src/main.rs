#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // show logs when debugging

mod core;
mod gui;
mod simple;
use crate::core::panic_handler::panic::set_panic_hook;
pub mod font;
pub mod icon;
mod logger;

fn main() {
    logger::win_attach_terminal();
    set_panic_hook();
    logger::init_logging();

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        gui::App::start()
    } else {
        simple::rip(args)
    }
}
