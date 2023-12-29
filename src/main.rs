#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::needless_lifetimes)]

pub mod app;
mod cli;
pub mod dialog;
pub mod event;
pub mod font;
pub mod icon;
pub mod logger;
pub mod ripper;
pub mod screen;
pub mod theme;
pub mod utils;
pub mod widget;

use app::XMODITS;
use cli::Mode;
use std::env;

#[cfg(all(feature = "jemallocator", not(target_env = "msvc")))]
use jemallocator::Jemalloc;

#[cfg(all(feature = "jemallocator", not(target_env = "msvc")))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() -> iced::Result {
    #[cfg(windows)]
    logger::reattach_windows_terminal();
    logger::set_panic_hook();

    let mut args: Vec<_> = env::args().collect();
    args.remove(0);

    match cli::parse(args) {
        Mode::None => XMODITS::launch().map(|_| tracing::info!("Bye :)")),
        #[cfg(windows)]
        Mode::DragNDrop(paths) => XMODITS::launch_simple(paths),
        Mode::Version => Ok(cli::print_version()),
        Mode::Help => Ok(cli::print_help()),
        #[cfg(feature = "built")]
        Mode::BuildInfo => Ok(cli::print_info()),
        Mode::Unrecognised(option) => Ok(cli::print_unrecognised(option)),
    }
}
