#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // show logs when debugging
#![allow(dead_code)]

pub mod app;
pub mod dialog;
pub mod font;
pub mod icon;
pub mod logger;
pub mod sample_ripper;
pub mod screen;
pub mod theme;
pub mod utils;
pub mod widget;
// mod simple;

use std::env;

fn main() -> iced::Result {
    let args = env::args().skip(1);

    let version = args
        .peekable()
        .next()
        .map(|a| a == "--version" || a == "-V")
        .unwrap_or_default();

    if version {
        println!("XMODITS {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // let args: Vec<String> =  env::args().collect();

    app::XMODITS::launch().map(|_| tracing::info!("Bye :)"))
}