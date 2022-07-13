#![windows_subsystem = "windows"]
mod app;
use std::env;
use std::path::PathBuf;
use xmodits_lib::Error;

#[cfg(feature="ascii_art")] const LOGO: &str = include_str!("../../extras/ascii_art.txt");
#[cfg(not(feature="ascii_art"))] const LOGO: &str = "xmodits";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const HELP: &str = "
USAGE:
  xmodits <module>... [destination folder]

FLAGS:
  -h, --help            Prints help information
  -v, --version         Prints version

EXAMPLES:
    xmodits song1.s3m

    xmodits song1.s3m ~/Downloads/

    xmodits song1.s3m song2.it 

    xmodits song1.s3m song2.it ~/Downloads/
";

fn help() {
    println!("{LOGO}-{VERSION}");
    println!("By {AUTHOR}");
    println!("{HELP}");
}

fn version() {
    println!("{VERSION}");
}

fn total_size_MB(paths: &Vec<PathBuf>) -> u64 {
    paths
        .iter()
        .map(|e| if let Ok(m) = e.metadata() { m.len() } else { 0 })
        .sum::<u64>() / (1024 * 1024)
}

fn main() -> Result<(), Error> {
    let args: Vec<std::ffi::OsString> = env::args_os().skip(1).collect();

    if args.len() == 0 { help(); return Ok(()); }

    #[cfg(unix)]
    match args[0].to_str() {
        Some("-v") | Some("--version")  => { return Ok(version()); },
        Some("-h") | Some("--help")     => { return Ok(help()); },
        _ => {},
    }

    let mut paths: Vec<PathBuf> = args
        .iter()
        .map(|f| PathBuf::from(f))
        .collect();
    
    #[cfg(unix)]{ 
    let total_size_mb = total_size_MB(&paths);
    if total_size_mb > 32 {
        println!("Ripping {} MB worth of trackers, please wait...", total_size_mb);
    }}

    let dest_dir: PathBuf = match paths.last().unwrap() {
        p if p.is_dir() && paths.len() > 1 => paths.pop().unwrap(),
        _ => env::current_dir()?,
    };

    let modules: Vec<&PathBuf> = paths
        .iter()
        .filter(|f| f.is_file())
        .collect();    

    if modules.len() == 0 { return Ok(help()); }
    
    modules.iter().for_each(|mod_path| {
        let e = app::dump_samples(mod_path, &dest_dir);
        #[cfg(unix)]
        if e.is_err()  {
            eprintln!("Error: {}", e);
        }
    });

    Ok(())
} 