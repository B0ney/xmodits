// #![windows_subsystem = "windows"]
mod app;
use std::path::PathBuf;
use xmodits_lib::Error;
use clap::{Command, arg, crate_version, crate_authors};

fn main() -> Result<(), Error> {
    let matches = Command::new(
            if cfg!(feature="ascii_art") {
                include_str!("../../extras/ascii_art.txt")
            } else {
                "xmodits"
            }            
        )
        .about("Sample dumping tool for tracker modules.")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            arg!(<module> "Path to tracker module, the last argument is the destination folder.")
                .required(true)
                .multiple_values(true)
        )
        .get_matches();

    let mut paths: Vec<PathBuf> = matches.get_many::<String>("module")
        .unwrap()
        .cloned()
        .map(|f| PathBuf::from(f))
        .collect();

    let dest_dir: PathBuf = match paths.last().unwrap() {
        p if p.is_dir() && paths.len() > 1 => paths.pop().unwrap(),
        _ => std::env::current_dir()?,
    };
    
    paths
        .iter()
        .filter(|f| f.is_file())
        .for_each(|mod_path| {
            if let Err(e) = app::dump_samples(mod_path, &dest_dir) {
                eprintln!("{}", e);
            }
        }
    );

    Ok(())
} 