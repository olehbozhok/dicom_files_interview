use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Command-line utility to catalog the patients in a drive
#[derive(Parser)]
pub struct Cli {
    /// The path to the folder or file to read
    pub path: PathBuf,
    /// Count of reader workers, may be usefull to increase value, if you use ssd hard drive.
    #[arg(default_value_t = 1)]
    #[arg(short, long)]
    pub num_workers: usize,
    /// Set the output format to TEXT or CSV.
    #[arg(value_enum)]
    #[arg(default_value_t = OutputFormat::Text)]
    #[arg(short, long)]
    pub output_format: OutputFormat,
    /// Specify a value to save the result to a file. By default, the result is redirected to stdout.
    #[arg(short, long)]
    pub result_filepath: Option<PathBuf>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    Text,
    Csv,
}

//using this function it is possible to add using crate config without changing main function
pub fn get_config() -> Cli {
    Cli::parse()
}
