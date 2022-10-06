#[cfg(feature="ascii_art")] const LOGO: &str = include_str!("../../extras/ascii_art.txt");
#[cfg(not(feature="ascii_art"))] const LOGO: &str = "xmodits";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
pub const HELP: &str = "
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

pub fn help() {
    println!("{LOGO}-{VERSION}");
    println!("By {AUTHOR}");
    println!("{HELP}");
}

pub fn version() {
    println!("{VERSION}");
}

pub fn total_size_MB(paths: &[std::path::PathBuf]) -> u64 {
    paths
        .iter()
        .map(|e| if let Ok(m) = e.metadata() { m.len() } else { 0 })
        .sum::<u64>() / (1024 * 1024)
}
