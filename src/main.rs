mod recast;

use std::fs;
use std::path::Path;

const CHUNK_SIZE: usize = 4 * 1024 * 1204; // 4 MiB

fn main() {
    let path = Path::new("src/main.rs");
    let bytes = match fs::read(path) {
        Err(why) => panic!("Could not read from {}: {}", path.display(), why),
        Ok(bytes) => bytes
    };
    let chunks = recast::split(bytes.as_slice(), CHUNK_SIZE);
    let merged = recast::merge(chunks);
    assert_eq!(merged, bytes);
}