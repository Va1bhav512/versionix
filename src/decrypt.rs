use std::io::{self, Cursor, BufRead, BufReader};
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

//pub fn read_commit(commit: &str) -> Result<(), io::Error> {
//    let commit_folder: String = ".vx/commits/";
//    commit_folder.push_str(commit.chars().take(2).collect());
//    let commit_path = &commit_folder;
//    commit_path.push_str(commit.chars().skip(2).collect());
//
//    let file = File::open(&commit_path)?;
//
//    let reader = BufReader::new(file);
//
//    let mut lines = reader.lines();
//
//    if let Some(commit_message) = 
//
//
//}
