#![windows_subsystem = "windows"]
mod app;
use std::env;
use std::path::{PathBuf, Path};
use xmodits_lib::Error;
mod api;

#[cfg(unix)] 
mod cli;
#[cfg(unix)]
mod app_unix;

#[cfg(target_os = "windows")]
mod app_win;
#[cfg(target_os = "windows")]
mod dialoge;

// fn main() -> Result<(), Error> {
//     // Collect arguments
//     let args: Vec<std::ffi::OsString> = env::args_os().skip(1).collect();

//     // Show help to user if they launch the app with no arguments
//     // On Windows, this is a dialogue box
//     if args.len() == 0 { 
//         #[cfg(windows)]{ return Ok(dialoge::show_help_box()); }

//         #[cfg(unix)]{ return Ok(cli::help()); }
//     }

//     // On *nix systems we quit the application if given:
//     // -v, --version, -h, --help
//     #[cfg(unix)]{ app_unix::check_args(&args); }

//     // Convert argument into a Vector of paths
//     let mut paths: Vec<PathBuf> = args
//         .iter()
//         .map(|f| PathBuf::from(f))
//         .collect();
    
//     // We treat the last argumet as the destination folder
//     // If the last argument is not a valid folder, make the destination
//     // folder the current executable directory.
//     let dest_dir: PathBuf = match paths.last().unwrap() {
//         p if p.is_dir() && paths.len() > 1 => paths.pop().unwrap(),
//         _ => env::current_dir()?,
//     };
    
//     // Filter paths to just contain files.
//     let modules: Vec<PathBuf> = paths
//         .iter()
//         .filter(|f| f.is_file())
//         .map(|f| f.clone())
//         .collect();   

//     if modules.len() == 0 { 
//         #[cfg(windows)]{ return Ok(dialoge::no_valid_modules()); }

//         #[cfg(unix)]{ return Ok(cli::help()); }
//     }

//     #[cfg(unix)]
//     return app_unix::run(&modules, &dest_dir); 

//     #[cfg(windows)]
//     return app_win::run(modules, dest_dir);
// }



use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = "cheese")] // TODO
pub struct Cli {
    #[arg(help="Trackers to rip, the last element can be a folder to place rips. E.g \"./music.s3m ./music.it ./dumps/\"")]
    #[clap(required = true)]
    trackers: Vec<PathBuf>,

    #[arg(help="Only name samples with an index. E.g. 01.wav")]
    #[arg(short='i', long)]
    index_only: bool,

    #[arg(help="Preserve sample indexing")]
    #[arg(short='r', long)]
    index_raw: bool,

    #[arg(help="Pad index with preceding 0s. e.g. 001, or 0001")]
    #[arg(default_value_t = 2, short='p', long="index-padding", value_parser=0..=5)]
    index_padding: i64,

    #[arg(help="Include embedded text from tracker (if it exists)")]
    #[arg(short='c', long)]
    with_comment: bool,

    #[arg(help="Don't create a new folder for samples")]
    #[arg(short, long)]
    no_folder: bool,

    #[arg(help="Name samples in UPPER CASE")]
    #[arg(short, long="upper", conflicts_with="lower_case", conflicts_with="index_only")]
    upper_case: bool,

    #[arg(help="Name samples in lower case")]
    #[arg(short, long="lower", conflicts_with="upper_case", conflicts_with="index_only")]
    lower_case: bool,

    #[cfg(feature="advanced")]
    #[arg(help="Rip samples in parallel")]
    #[arg(short='k', long)]
    parallel: bool,
}

fn main() {
    let cli = Cli::parse();
    if false {

    } else {

    } 

    #[cfg(feature="advanced")]
    if cli.parallel {
        api::rip_parallel(cli);
        return;
    }


    // dbg!(cli::total_size_MB(&cli.trackers));

    // dbg!(&cli.with_comment);
    // dbg!(&cli.trackers);
    
    api::rip(cli);    
}