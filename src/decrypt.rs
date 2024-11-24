use std::io::{self, Cursor, BufRead, BufReader, Error, ErrorKind, Write};
use zstd::stream::decode_all;
use std::fs::{read, read_to_string, File, create_dir_all};
use std::path::Path;

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
            Ok(tree_name_and_hash) => {
                match split_string(&tree_name_and_hash) {
                    Some((tree_name, tree_hash)) => {
                        // make tree with name tree name and create all files inside it
                        // Visit this tree in .vx/tree/
                        // println!("Tree: {}", tree_name);
                        visit_tree(&tree_name, tree_hash)?;

                    }
                    None => {
                        eprintln!("Something wrong with tree structure in commit! {}", commit);
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid file structure in commit!"));
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    Ok(())
}

fn visit_tree(tree_name: &str, tree_hash: String) -> Result<(), io::Error> {
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
            Ok(file_or_folder_name_and_hash) => {
                // Print file name (before \t) and then read the file and print its contents

                match split_string(&file_or_folder_name_and_hash) {
                    Some((file_or_folder_name, file_or_folder_hash)) => {
                        // Check whether file or folder and handle accordingly
                        if file_or_folder_name.starts_with("tree:") {

                            let folder_name = &file_or_folder_name[5..];
                            visit_tree(&folder_name, file_or_folder_hash)?;

                        } else if file_or_folder_name.starts_with("file:") {
                        // Print the file name and then visit its hash
                        // Make a new file with name file name and contents will be written by
                        // visit_file
                            let file_name = &file_or_folder_name[5..];
                            let mut file_path = String::new();
                            file_path.push_str(&tree_name);
                            file_path.push_str("\\");
                            file_path.push_str(&file_name);
                            // println!("File: {}", file_name);
                            // println!("File path: {}", file_path);
                            visit_file(&file_path, file_or_folder_hash)?;
                        }
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


fn visit_file(file_path: &str, file_hash: String) -> Result<(), io::Error> {
    let mut file_folder: String = ".vx/objects/".to_string();
    file_folder.push_str(&file_hash.chars().take(2).collect::<String>());
    let mut file_folder_file = file_folder.clone();
    file_folder_file.push_str("/");
    file_folder_file.push_str(&file_hash.chars().skip(2).collect::<String>());
    file_folder_file.push_str(".hash");

    let bytes = read(file_folder_file)?;

    let data = decompress_string(bytes);
    // println!("Decompressed successfully!");

    let parent_dir = Path::new(file_path).parent();

    if let Some(parent) = parent_dir {
        create_dir_all(parent)?;
    }
    // println!("Parent directories successfully created if not available!");

    let mut file = File::create(file_path)?;
    // println!("File created/overwritten successfully!");

    file.write_all(data.as_bytes())?;

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
