use std::io::{self, Cursor, Read, Write, Error, ErrorKind};
use std::fs::{File, create_dir_all, OpenOptions, read_dir};
use std::path::{Path,PathBuf};
use zstd::stream::encode_all;
use sha2::{Sha256, Digest};

pub fn visit_dirs(dir: &Path, commit: &mut String) -> Result<(), io::Error> {
    println!("visting directories at: {:?}", dir);
    if let Some(dir_str) = dir.to_str() {
        commit.push_str(dir_str);
        commit.push_str("\t");
        // let dir_string = dir_str.to_string();
    }
    let mut tree = String::new();
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // function to call to do 5. of to-do.
                //if let Some(parent) = path.parent() {
                //    if let Some(folder_name) = parent.file_name() {
                //        if let Some(folder_name_str) = folder_name.to_str() {
                //            tree.push_str(folder_name_str);
                //        }
                //    }
                //}
                // visit_dirs(&path, commit)?;
                println!("{:?} is a directory", path);
                visit_inner_dirs(path, &mut tree)?;
            } else if path.is_file() {
                // function to encrypt it.
                println!("{:?} is a file, hashing:", path);
                hash_file(path, &mut tree)?;

            }
        }
    }

    let mut hasher = Sha256::new();
    hasher.update(tree.as_bytes());
    let result = hasher.finalize();
    let hash = format!("{:x}", result);
    store_tree(&tree,&hash)?;
    commit.push_str(&hash);
    commit.push_str("\n");
    println!("commit: {}", commit);
    Ok(())
}

fn visit_inner_dirs(pathbuf: PathBuf, parent_tree: &mut String) -> Result<(), io::Error> {
    let dir = pathbuf.as_path();
    println!("visiting inner directories at :{:?}", dir);
    let mut tree = String::new();
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_inner_dirs(path.clone(), &mut tree)?;
            } else if path.is_file() {
                println!("{:?} is a file, hashing", path);
                hash_file(path.clone(), &mut tree)?;
            }
        }
    }
    let mut hasher = Sha256::new();
    hasher.update(tree.as_bytes());
    let result = hasher.finalize();
    let hash = format!("{:x}", result);
    store_tree(&tree,&hash)?;

    //if let Some(parent) = dir.parent() {
    //    if let Some(folder_name) = parent.file_name() {
    //        if let Some(folder_name_str) = folder_name.to_str() {
    //            println!("HEREEEEEEEEEEEEEEEEEEEEEE {}", folder_name_str);
    //            parent_tree.push_str(folder_name_str);
    //        } else {
    //          return Err(Error::new(ErrorKind::InvalidData, "Path name for inner tree not valid!"));
    //        }
    //    } else {
    //      return Err(Error::new(ErrorKind::InvalidData, "Path name for inner tree not valid!"));
    //    }
    //}
    
    if let Some(path_str) = dir.to_str() {
        parent_tree.push_str("tree:");
        parent_tree.push_str(path_str);
    } else {
        return Err(Error::new(ErrorKind::InvalidData, "Path name for inner tree not valid!"));
    }

    parent_tree.push_str("\t");
    parent_tree.push_str(&hash);
    parent_tree.push_str("\n");
    println!("parent_tree: {}", parent_tree);
    Ok(())

}

pub fn store_commit(commit: &str, commit_message: &str) -> Result<(), io::Error> {

    let mut hasher = Sha256::new();
    hasher.update(commit.as_bytes());
    let result = hasher.finalize();
    let hash = format!("{:x}", result);

    let first_two_chars: String = hash.chars().take(2).collect();
    let rest_of_chars: String = hash.chars().skip(2).collect();
    let path = format!(".vx/commits/{}",first_two_chars);
    println!("path of commit: {}", path);
    create_dir_all(path.clone())?;
    let file_path = format!("{}/{}.commit",path,rest_of_chars);
    println!("path of commit: {}", file_path);
    let mut file = File::create(file_path)?;

    println!("Writing commit to file:");
    file.write_all(&commit.as_bytes())?;

    // Writing commit hash to history file

    let mut file = File::open(".vx/history.history")?;
    let mut existing_commits = String::new();
    file.read_to_string(&mut existing_commits)?;
    let combined_data = format!("{}\t{}\n{}", commit_message, hash, existing_commits);
    println!("combined data {}", combined_data);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(".vx/history.history")?;

    file.write_all(combined_data.as_bytes())?;
    Ok(())
}

fn hash_file(pathbuf: PathBuf, tree: &mut String) -> Result<(), io::Error> {
    let path = pathbuf.as_path();
    if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            tree.push_str("file:");
            tree.push_str(&file_name_str);
            tree.push_str("\t");
        }
    }
    let mut hasher = Sha256::new();
    let mut file = File::open(path)?;
    // let metadata = "blob.";

    let mut object = String::new();
    // object.push_str(metadata);

    hasher.update(object.as_bytes());

    let mut buffer = [0u8; 4096];  // 4KB buffer

    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;  // End of file
        }
        let file_chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
        object.push_str(&file_chunk);
    }

    hasher.update(object.as_bytes());
    let result = hasher.finalize();
    
    let hash = format!("{:x}",result);
    println!("object: {}", object);
    println!("hash: {}", hash);
    println!("Now storing:");
    println!("Appending hash to tree object:");
    tree.push_str(&hash);
    tree.push_str("\n");
    println!("tree: {}", tree);


    store(object, hash)?;

    Ok(())
}



fn store(object: String, hash: String) -> Result<(), io::Error> {
    let first_two_chars: String = hash.chars().take(2).collect();
    let rest_of_chars: String = hash.chars().skip(2).collect();
    let path = format!(".vx/objects/{}",first_two_chars);

    println!("path: {}",path);

    create_dir_all(path.clone())?;

    let file_path = format!("{}/{}.hash",path,rest_of_chars);

    println!("file_path: {}", file_path);

    let mut file = File::create(file_path)?;

    let compressed_string = compress_string(object);

    println!("Now writing to file:");

    file.write_all(&compressed_string)?;

    Ok(())

}
fn store_tree(tree: &String, hash: &String) -> Result<(), io::Error> {
    let first_two_chars: String = hash.chars().take(2).collect();
    let rest_of_chars: String = hash.chars().skip(2).collect();
    let path = format!(".vx/tree/{}",first_two_chars);

    println!("path of tree: {}", path);

    create_dir_all(path.clone())?;

    let file_path = format!("{}/{}.tree",path,rest_of_chars);

    println!("path of tree file: {}", file_path);

    let mut file = File::create(file_path)?;

    println!("Writing tree to file:");

    file.write_all(&tree.as_bytes())?;

    Ok(())
}


fn compress_string(input: String) -> Vec<u8> {
    let compressed_data = encode_all(Cursor::new(input.as_bytes()), 3).unwrap();
    compressed_data
}


