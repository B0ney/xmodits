#![windows_subsystem = "windows"]
mod xmodits_api;
mod cfg;
mod app;
use std::env;

fn main() {
    // The user may want to just drag and drop a module
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.is_empty() {
        app::launch();
    } else {
        xmodits_api::rip_simple(args);
    }
}
