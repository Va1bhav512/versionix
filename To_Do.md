1. Compression
2. Hashing
3. Making a folder like .vx
4. Reading contents of every file
5. Need a way like if I visit file I encrypt it using the sha-1 (or equivalent function) and if I visit a folder then I encrypt it and make
   it point to the tree structure of this folder. And call the function again to encrypt files and folders inside it?
6. Update the metadata to be useful.




Optimize hashing for very large files
```rs
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let file_path = "path/to/your/file.txt";
    let mut file = File::open(file_path)?;

    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 4096]; // Buffer for 4 KB

    // Read the file in fixed-size chunks
    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break; // End of file
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Get the final SHA-256 hash
    let result = hasher.finalize();
    println!("SHA-256 hash: {:x}", result);

    Ok(())
}
```
