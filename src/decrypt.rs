use std::io::{self, Cursor, BufRead, BufReader, Error, ErrorKind};
use zstd::stream::decode_all;
use std::fs::{read, read_to_string, File};

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

pub fn read_commit(commit: &String) -> Result<(), io::Error> {
    let mut commit_folder: String = ".vx/commits/".to_string();
    commit_folder.push_str(&commit.chars().take(2).collect::<String>());
    let mut commit_path = commit_folder.clone();
    commit_path.push_str("/");
    commit_path.push_str(&commit.chars().skip(2).collect::<String>());
    commit_path.push_str(".commit");

    let file = File::open(&commit_path)?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(tree_hash) => {
                // Visit this tree in .vx/tree/
                visit_tree(tree_hash)?;
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    Ok(())
}

fn visit_tree(tree_hash: String) -> Result<(), io::Error> {
    let mut tree_folder: String = ".vx/tree/".to_string();
    tree_folder.push_str(&tree_hash.chars().take(2).collect::<String>());
    let mut tree_folder_file = tree_folder.clone();
    tree_folder_file.push_str("/");
    tree_folder_file.push_str(&tree_hash.chars().skip(2).collect::<String>());
    tree_folder_file.push_str(".tree");

    let file = File::open(&tree_folder_file)?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(file_name_and_hash) => {
                // Print file name (before \t) and then read the file and print its contents

                match split_string(&file_name_and_hash) {
                    Some((file_name, file_hash)) => {
                        // Print the file name and then visit its hash
                        println!("File: {}", file_name);
                        visit_file(file_hash)?;
                    }
                    None => {
                        eprintln!("Some thing wrong with file structure in tree! {}", tree_hash);
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid file structure in tree!"));
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading file name and hash: {}", e);
            }
        }
    }

    Ok(())
}


fn visit_file(file_hash: String) -> Result<(), io::Error> {
    let mut file_folder: String = ".vx/objects/".to_string();
    file_folder.push_str(&file_hash.chars().take(2).collect::<String>());
    let mut file_folder_file = file_folder.clone();
    file_folder_file.push_str("/");
    file_folder_file.push_str(&file_hash.chars().skip(2).collect::<String>());
    file_folder_file.push_str(".hash");

    let bytes = read(file_folder_file)?;

    let data = decompress_string(bytes);

    println!("{}",data);

    Ok(())
}

fn split_string(input: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = input.splitn(2,'\t').collect();

    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}
