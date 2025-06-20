use std::fs;
use std::io;
use sted::path::{Path, PathBuf};

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
            track_now: usize,
        };
        fetching.load_tracks(folder);
        fetching
    }
}
