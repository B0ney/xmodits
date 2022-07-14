use std::path::Path;
use std::path::PathBuf;

use xmodits_lib::Error;
use crate::app;
use crate::cli;
use cli::{ version, help, total_size_MB };

pub fn check_args(args: &[std::ffi::OsString]) {
    match args[0].to_str() {
        Some("-v") | Some("--version")  => { version(); std::process::exit(0) },
        Some("-h") | Some("--help")     => { help(); std::process::exit(0) },
        _ => {},
    }
}

pub fn run(modules: &[PathBuf], dest_dir: &PathBuf) -> Result<(), Error> {
    if modules.len() == 0 { 
        return Ok(help()); 
    }

    let total_size_mb: u64 = total_size_MB(&modules);

    if total_size_mb > 32 {
        println!("Ripping {} MB worth of trackers, please wait...", total_size_mb);
    }

    modules.iter().for_each(|mod_path| {
        if let Err(e) = app::dump_samples(mod_path, &dest_dir) {
            eprintln!("Error: {:?}", e);
        }
    });

    Ok(())

}