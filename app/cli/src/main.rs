use std::path::PathBuf;
use clap::Parser;

use xmodits::{Cli, api};

fn main() {
    let mut cli = Cli::parse();

    let destination: PathBuf = match cli.trackers.last().unwrap() {
        p if !p.is_file() && cli.trackers.len() > 1 => {
            let folder = cli.trackers.pop().unwrap();

            if !folder.is_dir() {
                if let Err(e) = std::fs::create_dir(&folder) {
                    return eprintln!(
                        "Error: Could not create destination folder \"{}\": {}",
                        folder.display(),
                        e
                    );
                };
            }

            folder
        }
        _ => std::env::current_dir().expect("I need a current working directory. (>_<)"),
    };
    if cli.info {
        return api::info(cli);
    }

    #[cfg(feature = "advanced")]
    if cli.parallel {
        return api::rip_parallel(cli, destination);
    }   

    api::rip(cli, destination);

    #[cfg(windows)]
    {
        use std::io::{stdin, stdout, Write};
        let mut buf = String::new();
        print!("\nPress Enter to continue... ");
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut buf);
        let _ = stdout().flush();
    }
}
