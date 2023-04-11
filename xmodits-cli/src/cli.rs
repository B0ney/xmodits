use clap::Parser;
use std::path::PathBuf;

const ABOUT: &str = "A tool to rip samples from tracker music. Supports IT, XM, S3M, MOD and UMX formats.\nThis software is licensed under the LGPLv3, for more infomation please visit: https://github.com/B0ney/xmodits";

#[derive(Parser)]
#[command(author, version, about)]
#[command(long_about = ABOUT)]
pub struct Cli {
    #[arg(
        help = "Modules to rip, the last element can be a folder to place your rips. E.g \"./music.s3m ./music.it ./dumps/\""
    )]
    #[arg(required = true)]
    pub trackers: Vec<PathBuf>,

    #[arg(help = "Only name samples with an index. E.g. 01.wav")]
    #[arg(conflicts_with = "upper_case", conflicts_with = "lower_case")]
    #[arg(short = 'i', long)]
    pub index_only: bool,

    #[arg(help = "Preserve sample indexing")]
    #[arg(short = 'r', long)]
    pub index_raw: bool,

    #[arg(help = "Pad index with preceding zeros. E.g. 001, or 0001")]
    #[arg(default_value_t = 2, short='p', long="index-padding", value_parser=0..=5)]
    pub index_padding: i64,

    #[arg(help = "Do not create a new folder for samples. This can overwrite data, BE CAREFUL!")]
    #[arg(short, long)]
    pub no_folder: bool,

    #[arg(help = "Name samples in upper case")]
    #[arg(short, long = "upper", conflicts_with = "lower_case")]
    pub upper_case: bool,

    #[arg(help = "Name samples in lower case")]
    #[arg(short, long = "lower", conflicts_with = "upper_case")]
    pub lower_case: bool,

    #[arg(help = "Prefix samples with the filename")]
    #[arg(short = 'g', long)]
    pub prefix: bool,

    #[arg(help = "Export formats")]
    #[arg(short, long = "fmt", value_parser=["wav", "aiff", "8svx", "raw"], default_value_t=String::from("wav"))]
    pub format: String,

    #[arg(help = "Print information about a tracker module")]
    #[arg(long)]
    pub info: bool,

    #[cfg(feature = "rayon")]
    #[arg(help = "Rip samples in parallel")]
    #[arg(short = 'k', long, default_value_t = 0)]
    pub threads: u8,
}
