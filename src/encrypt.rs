use std::io::{self, Cursor, Read, Write};
use std::fs::{self, File, create_dir_all};
use std::path::{Path,PathBuf};
use zstd::stream::{encode_all, decode_all};
use sha2::{Sha256, Digest};

pub fn visit_dirs(dir: &Path) -> io::Result<()> {
    println!("visting directories at: {:?}", dir);
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // function to call to do 5. of to-do.
                println!("{:?} is a directory", path);
            } else if path.is_file() {
                // function to encrypt it.
                println!("{:?} is a file, hashing:", path);
                hash_file(path);

            }
        }
    }
    Ok(())
}

// fn encode_data(dir: &Path, path: &Path) -> io::Result<()> {
//     let input_file = File::open(path)?;
//     let output_file = File::create(".vx/objects/output.txt.zst")?;
//     let mut file_contents = Vec::new();
//     input_file.read_to_end(&mut file_contents);
//     let path_dict = path.as_bytes();
//     let encoder_dict = EncoderDict::new(path_dict);
//     let mut encoder = Encoder::new(output_file, 3);
//     encoder.write_all(&file_contents);
//     encoder.finish()?;
//     Ok(())
// }







fn hash_file(pathbuf: PathBuf) -> io::Result<()> {
    let path = pathbuf.as_path();
    let mut hasher = Sha256::new();
    let mut file = File::open(path)?;
    let metadeta = "blob.";

    let mut object = String::new();
    object.push_str(metadeta);

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
    
    // object, hash = (object, format!("{:x}",result))
    
    let hash = format!("{:x}",result);
    println!("object: {}", object);
    println!("hash: {}", hash);
    println!("Now storing:");


    store(object, hash);


    // Convert the hash to a hexadecimal string
    Ok(())
}



fn store(object: String, hash: String) -> io::Result<()> {
    let first_two_chars: String = hash.chars().take(2).collect();
    let rest_of_chars: String = hash.chars().skip(2).collect();
    let path = format!(".vx/objects/{}",first_two_chars);

    println!("path: {}",path);

    create_dir_all(path.clone())?;

    let file_path = format!("{}/{}.hash",path,rest_of_chars);

    println!("file_path: {}", file_path);

    let mut file = File::create(file_path)?;

    let compressed_string = compress_string(object);

    // println!("compressed_string: {}", compressed_string);

    println!("Now writing to file:");

    file.write_all(&compressed_string);

    Ok(())

}


fn compress_string(input: String) -> Vec<u8> {
    let compressed_data = encode_all(Cursor::new(input.as_bytes()), 3).unwrap();
    compressed_data
}


fn decompress_string(compressed_data: Vec<u8>) -> String {
    let decompressed = decode_all(Cursor::new(compressed_data)).unwrap();
    String::from_utf8(decompressed).unwrap()
}
