//! File Upload Logic

use std::{path::Path, fs};
use tokio::fs::File;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use common::{FileHeader, Command, VeriflowError};
use crate::{hashing, ui};

pub async fn upload_file(path: &Path, ip: &str) -> common::Result<()> {
  // Offline Logic (Validation)
  
  // get file with tokio (VeriflowError if it doesn't exist)
  let mut file = File::open(path).await?;

  // get file metadata
  let file_metadata = file.metadata().await?;
  let file_size = file_metadata.len();

  
  // get file name -- Strict error handling (Allow ONLY UTF-8 characters)
  let file_name = path.file_name()
    .and_then(|name| name.to_str())
    .ok_or(VeriflowError::InvalidPath)?;

  // Hashing
  println!("Starting Hashing...");
  let file_hash = hashing::hash_file(path).await?;

  println!("File Hash: {file_hash}");


  // Connect to server
  
  // connect via TCP stream
  let mut stream = TcpStream::connect(ip).await?;

  // Setup FileHeader
  let file_header: FileHeader = FileHeader {
      command: Command::Upload,
      name: String::from(file_name),
      size: file_size,
      hash: String::from(file_hash),
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

  println!("length {json_len}, prefix bytes: {:?}", &prefix_bytes);

  // Send the actual file header
  stream.write_all(json_bytes).await?;

  // stream the body (file upload)
  
  Ok(())
}