#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // show logs when debugging

mod core;
mod gui;
mod simple;

pub mod dialog;
pub mod font;
pub mod icon;
pub mod logger;
pub mod theme;

fn main() {
    logger::win_attach_terminal();
    logger::set_panic_hook();
    logger::init_logging();

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        gui::App::start()
    } else {
        simple::rip(args)
    }
}
