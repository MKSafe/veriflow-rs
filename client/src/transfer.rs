//! File Upload Logic

use crate::{hashing, ui};
use common::{Command, FileHeader, VeriflowError};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

// convention: 4096B or 8192B
// Buffer size of 8kb for TCP
const BUFFER_SIZE: usize = 4096;

pub async fn upload_file(path: &Path, ip: &str) -> common::Result<()> {
    // Offline Logic (Validation)

    // get file with tokio (VeriflowError if it doesn't exist)
    let mut file = File::open(path).await?;

    // get file metadata
    let file_metadata = file.metadata().await?;
    let file_size = file_metadata.len();

    // get file name -- Strict error handling (Allow ONLY UTF-8 characters)
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or(VeriflowError::InvalidPath)?;

    // Hashing
    println!("Starting Hashing...");
    let file_hash = hashing::hash_file(path).await?;

    println!("File Hash: {file_hash}");

    // Connect to server
    println!("Connecting to {ip}...");

    // connect via TCP stream
    let mut stream = TcpStream::connect(ip).await?;

    // Setup FileHeader
    let file_header: FileHeader = FileHeader {
        command: Command::Upload,
        name: String::from(file_name),
        size: file_size,
        hash: file_hash,
    };

    // Serialise the body
    // JSON string
    let header_json = serde_json::to_string(&file_header)?;

    // Convert to raw bytes
    let json_bytes = header_json.as_bytes();

    //  Get prefix length as u32
    let json_len = json_bytes.len() as u32;
    // Use big endian due to network standard (old server cpus)
    let prefix_bytes = json_len.to_be_bytes();

    // Send the prefix
    stream.write_all(&prefix_bytes).await?;

    // println!("length {json_len}, prefix bytes: {:?}", &prefix_bytes);

    // Send the actual file header
    stream.write_all(json_bytes).await?;

    // File Upload
    println!("Starting Upload...");

    // create progress bar
    // set max to len of file and operation description
    let progress_bar = ui::create_progress_bar(file_size, "Uploading ...");

    // Stream the body

    // Buffer
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    // read file using buffer
    loop {
        // Read chunk from file (number of bytes successfully read)
        let bytes_read: usize = file.read(&mut buffer).await?;

        // finish reading file
        if bytes_read == 0 {
            // break loop
            break;
        }

        // update progress bar
        progress_bar.inc(bytes_read as u64);

        // load the chunk from file
        let current_chunk: &[u8] = &buffer[..bytes_read];

        // update stream with current chunk reference
        stream.write_all(current_chunk).await?;
    }

    // finish progress bar
    progress_bar.finish_with_message("Upload Complete!");

    Ok(())
}
