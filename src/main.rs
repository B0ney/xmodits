#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::needless_lifetimes)]

pub mod app;
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
use std::env;

#[cfg(all(feature = "jemallocator", not(target_env = "msvc")))]
use jemallocator::Jemalloc;

#[cfg(all(feature = "jemallocator", not(target_env = "msvc")))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() -> iced::Result {
    #[cfg(windows)]
    logger::reattach_windows_terminal();

    let args = env::args().skip(1);

    let version = args
        .peekable()
        .next()
        .map(|a| a == "--version" || a == "-V")
        .unwrap_or_default();

    if version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    #[cfg(windows)]
    if env::args().len() > 1 {
        return XMODITS::launch_simple(env::args().skip(1));
    }

    XMODITS::launch().map(|_| tracing::info!("Bye :)"))
}
