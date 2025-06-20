use clap::Parser;
use std::path::PathBuf;

use crate::Fetcher as Fetch;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Quickest Crates In the West")]
struct Cli {
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let path = cli.path.unwrap_or(PathBuf::from("~/Music/INCOMING"));
    println!("{}", path.display());

    let example = Fetch.new();

    println!("{}", example)
}
