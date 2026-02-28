//! File hashing via SHA256

use sha2::{Digest, Sha256};
use std::io;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

// convention: 4096B or 8192B
// Buffer size of 8kb for hashing
const BUFFER_SIZE: usize = 4096;

// Hashes a file using SHA256
// Function now accepts a callback
pub async fn hash_file<F>(path: &Path, mut on_progress: F) -> io::Result<String>
where
    F: FnMut(usize),
{
    // Buffer
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    // get file with tokio
    let mut file = File::open(path).await?;

    // create hasher for SHA256
    let mut hasher: Sha256 = Sha256::new();

    // read file using buffer
    loop {
        // Read chunk from file (number of bytes successfully read)
        let bytes_read: usize = file.read(&mut buffer).await?;

        // finish reading file
        if bytes_read == 0 {
            // break loop
            break;
        }

        // trigger callback for progressbar
        on_progress(bytes_read);

        // load the chunk from file
        let current_chunk: &[u8] = &buffer[..bytes_read];

        // update hasher with current chunk reference
        hasher.update(current_chunk);
    }

    // finalise hasher, get its output (byte array)
    let file_hash = hasher.finalize();

    // Convert hash (byte array) to hex
    let file_hash_hex: String = format!("{:x}", file_hash);

    // Send back if successful
    Ok(file_hash_hex)
}
