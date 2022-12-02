use clap::Parser;
use xmodits_common::destination_dir;
mod api;
mod cli;
use cli::Cli;

fn main() {
    let mut cli = Cli::parse();

    let destination = match destination_dir(&mut cli.trackers) {
        Ok(path) => path,
        Err(e) => {
            return eprintln!("{}", e);
        }
    };

    if cli.info {
        return api::info(cli);
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
