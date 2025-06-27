use clap::Parser;
use scan_dir::ScanDir;
use std::path::PathBuf;

use crate::fetcher::Fetcher;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Quickest Crates In the West")]
struct Cli {
    path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let path = cli.path.unwrap_or(PathBuf::from("~/Music/INCOMING"));

    println!("I found some incoming tracks for you to sort....");
    println!("{}", path.display());

    ScanDir::dirs()
        .read("..", |iter| {
            for (entry, name) in iter {
                println!("{:?} -=-=-=- {:?}", name, entry.path());
            }
        })
        .unwrap()
}
