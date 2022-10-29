#![windows_subsystem = "windows"]
// mod app;
mod core;
mod gui;

use std::env;
fn main() {
    // The user may want to just drag and drop a module
    let args: Vec<String> = env::args().skip(1).collect();
    gui::XmoditsGui::start();
    // if args.is_empty() {
        
    // } else {
    //     xmodits_api::rip_simple(args);
    // }
}
