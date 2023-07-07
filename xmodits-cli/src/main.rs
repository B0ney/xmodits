use clap::Parser;
use std::path::PathBuf;
mod api;
mod cli;
use cli::Cli;

fn main() {
    let mut cli = Cli::parse_from(wild::args());

    let destination = match destination_dir(&mut cli.trackers) {
        Ok(path) => path,
        Err(e) => {
            return eprintln!("{}", e);
        }
    };

    if cli.info {
        return api::info(cli);
    }

    #[cfg(windows)]
    let no_exit_prompt = cli.no_exit_prompt;

    api::rip(cli, destination);

    #[cfg(windows)]
    if !no_exit_prompt {
        use std::io::{stdin, stdout, Write};
        let mut buf = String::new();
        print!("\nPress Enter to continue... ");
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut buf);
        let _ = stdout().flush();
    }
}

/// Checks if the last element in paths is a folder that exists.
///
/// If not, it will create that folder.
///
/// If the last element is a file, the destination directory is the
/// current working directory.
fn destination_dir(paths: &mut Vec<PathBuf>) -> Result<PathBuf, String> {
    let cwd = || {
        let error = String::from("xmodits needs a current working directory. (>_<)");
        Ok(std::env::current_dir().map_err(|_| error)?)
    };

    let Some(path) = paths.last() else {
        return cwd();
    };

    // Make sure path is NOT a file, and the length is over 1
    if path.is_file() || paths.len() <= 1 {
        return cwd();
    }

    let folder = paths.pop().unwrap();

    if !folder.is_dir() {
        if let Err(e) = std::fs::create_dir(&folder) {
            return Err(format!(
                "Error: Could not create destination folder \"{}\": {}",
                folder.display(),
                e
            ));
        };
    }

    Ok(folder)
}
