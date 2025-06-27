use std::fs;
use std::io;
use sted::path::{Path, PathBuf};

mod fetcher;

pub struct Fetcher {
    folder: PathBuf,
    files: Vec<PathBuf>,
    track_now: usize,
}

impl Fetcher {
    pub fn new(folder: PathBuf) -> Self {
        let mut fetching = Fetcher {
            folder: folder.clone(),
            files: Vec::new(),
            track_now: 0,
        };

        fetching
    }

    pub fn load_tracks() -> () {
        println!("you made it this far only.")
    }
}
