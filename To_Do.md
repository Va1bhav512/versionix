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

Optimize storing commits in history.history
```rs
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::env;

fn prepend_to_file(file_path: &str, new_content: &str) -> io::Result<()> {
    // Step 1: Open the original file for reading
    let mut file = File::open(file_path)?;

    // Step 2: Create a temporary file for the new content
    let temp_file_path = format!("{}.tmp", file_path); // Temporary file with a .tmp extension
    let mut temp_file = File::create(&temp_file_path)?;

    // Step 3: Write the new content to the temporary file
    temp_file.write_all(new_content.as_bytes())?;

    // Step 4: Copy the existing content from the original file to the temporary file
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?; // Read the entire content of the original file
    temp_file.write_all(&buffer)?; // Write the existing content to the temporary file

    // Step 5: Replace the original file with the temporary file
    std::fs::remove_file(file_path)?; // Delete the original file
    std::fs::rename(&temp_file_path, file_path)?; // Rename the temporary file to the original file name

    Ok(())
}

fn main() -> io::Result<()> {
    let file_path = "example.txt";
    let new_content = "This is the new content at the beginning.\n";

    prepend_to_file(file_path, new_content)?;

    println!("Content prepended successfully!");

    Ok(())
}

```


commit structure: hash of directories
