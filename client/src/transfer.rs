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

  // Hashing
  println!("Starting Hashing...");
  let file_hash = hashing::hash_file(path).await?;

  println!("File Hash: {file_hash}");


  // Connect to server


  Ok(())
}