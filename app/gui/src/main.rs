// #![windows_subsystem = "windows"] // Will this make logging impossible?
// #[allow(unused)]
mod core;
// #[allow(unused)]
mod gui;
// #[allow(unused)]
mod simple;
use std::env;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    // The user may want to just drag and drop a module
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.is_empty() {
        info!("Starting gui");
        gui::XmoditsGui::start();
    } else {
        simple::rip(args);
    }
}
