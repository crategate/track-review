use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Quickest Crates In the West")]
struct Cli {
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    println!("Hello, world!");

    let path = cli.path.unwrap_or(PathBuf::from("~/Music/INCOMING"));
}
