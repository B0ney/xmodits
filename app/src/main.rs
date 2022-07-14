#![windows_subsystem = "windows"]
mod app;
use std::env;
use std::path::PathBuf;
use xmodits_lib::Error;

#[cfg(unix)] 
mod cli;
#[cfg(unix)]
mod app_unix;

// #[cfg(target_os = "windows")]
mod app_win;
// #[cfg(target_os = "windows")]
mod dialoge;

fn main() -> Result<(), Error> {
    // Collect arguments
    let args: Vec<std::ffi::OsString> = env::args_os().skip(1).collect();

    // Show help to user if they launch the app with no arguments
    // On Windows, this is a dialogue box
    if args.len() == 0 { 
        #[cfg(windows)]{ return Ok(dialoge::show_help_box()); }

        #[cfg(unix)]{ return Ok(cli::help()); }
    }

    // On *nix systems we quit the application if given:
    // -v, --version, -h, --help
    #[cfg(unix)]{ app_unix::check_args(&args); }

    // Convert argument into a Vector of paths
    let mut paths: Vec<PathBuf> = args
        .iter()
        .map(|f| PathBuf::from(f))
        .collect();
    
    // We treat the last argumet as the destination folder
    // If the last argument is not a valid folder, make the destination
    // folder the current executable directory.
    let dest_dir: PathBuf = match paths.last().unwrap() {
        p if p.is_dir() && paths.len() > 1 => paths.pop().unwrap(),
        _ => env::current_dir()?,
    };
    
    // Filter paths to just contain files.
    let modules: Vec<PathBuf> = paths
        .iter()
        .filter(|f| f.is_file())
        .map(|f| f.clone())
        .collect();   

    if modules.len() == 0 { 
        #[cfg(windows)]{ return Ok(dialoge::no_valid_modules()); }
        #[cfg(unix)]{ return Ok(cli::help()); }
    }

    // #[cfg(unix)]
    // return app_unix::run(&modules, &dest_dir); 

    // #[cfg(windows)]
    return app_win::run(modules, dest_dir);
}