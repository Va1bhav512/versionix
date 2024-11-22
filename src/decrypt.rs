use std::io::{self, Cursor};
use zstd::stream::decode_all;
use std::fs::read_to_string;

fn decompress_string(compressed_data: Vec<u8>) -> String {
    let decompressed = decode_all(Cursor::new(compressed_data)).unwrap();
    String::from_utf8(decompressed).unwrap()
}

pub fn read_commit_history() -> Result<(), io::Error> {
    let file_path = ".vx/history.history";
    let commits = read_to_string(file_path)?;
    println!("{}", commits);
    Ok(())
}
