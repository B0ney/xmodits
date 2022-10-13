mod app;
mod api;
use std::env;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = "A tool to rip samples from tracker music. Supports IT, XM, S3M & MOD formats.\nhttps://github.com/B0ney/xmodits - GPLv3")]
pub struct Cli {
    #[arg(help="Modules to rip, the last element can be a folder to place your rips. E.g \"./music.s3m ./music.it ./dumps/\"")]
    #[arg(required = true)]
    trackers: Vec<PathBuf>,

    #[arg(help="Only name samples with an index. E.g. 01.wav")]
    #[arg(short='i', long, conflicts_with="upper_case", conflicts_with="lower_case")]
    index_only: bool,

    #[arg(help="Preserve sample indexing")]
    #[arg(short='r', long)]
    index_raw: bool,

    #[arg(help="Pad index with preceding 0s. e.g. 001, or 0001")]
    #[arg(default_value_t = 2, short='p', long="index-padding", value_parser=0..=5)]
    index_padding: i64,

    // #[arg(help="Include embedded text from tracker (if it exists)")]
    // #[arg(short='c', long)]
    // with_comment: bool,

    #[arg(help="Don't create a new folder for samples.")]
    #[arg(short, long)]
    no_folder: bool,

    #[arg(help="Name samples in UPPER CASE")]
    #[arg(short, long="upper", conflicts_with="lower_case")]
    upper_case: bool,

    #[arg(help="Name samples in lower case")]
    #[arg(short, long="lower", conflicts_with="upper_case")]
    lower_case: bool,

    #[cfg(feature="advanced")]
    #[arg(help="Rip samples in parallel")]
    #[arg(short='k', long)]
    parallel: bool,
}

fn main() {
    let mut cli = Cli::parse();

    let destination: PathBuf = match cli.trackers.last().unwrap() {
        p if !p.is_file() && cli.trackers.len() > 1 => {
            let folder = cli.trackers.pop().unwrap();

            if !folder.is_dir() {
                if let Err(e) = std::fs::create_dir(&folder) {
                    return eprintln!("Error: Could not create destination folder \"{}\": {}", folder.display(), e);
                };
            }

            folder
        },
        _ => env::current_dir().expect("I need a current working directory. (>_<)"),
    };

    #[cfg(feature="advanced")]
    if cli.parallel {
        api::rip_parallel(cli, destination);
        return;
    }

    api::rip(cli, destination);
}