use client::transfer;
use common::VeriflowError;
use server::server;

// External dependencies for tests
use std::path::Path;
use tempfile::tempdir;
use tokio::fs;

/// Client list command
#[tokio::test]
async fn integration_test_list_command() -> Result<(), VeriflowError> {
    // Setup temp dir
    let temp_dir = tempdir()?;
    let resource_path = temp_dir.path().to_path_buf();

    // Create fake file
    fs::File::create(resource_path.join("some_file.txt")).await?;

    // Initialise server
    let mut listener = server::Listener::new("127.0.0.1", "0").await?;
    let target_addr = listener.local_addr()?.to_string();

    // Spawn the server
    tokio::spawn(async move {
        let _ = listener.listen(resource_path).await;
    });

    // Wait until the server starts listening
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send list command
    transfer::list_files(&target_addr).await?;

    Ok(())
}

/// Upload download loop
#[tokio::test]
async fn integration_test_upload_download_roundtrip() -> Result<(), VeriflowError> {
    // Setup temp dir
    let server_dir = tempdir()?;
    let server_path = server_dir.path().to_path_buf();

    let upload_dir = tempdir()?;
    let upload_file = upload_dir.path().join("test.txt");
    fs::write(&upload_file, "Hello, test testing!").await?;

    // client download
    let download_dir = tempdir()?;
    let download_path = download_dir.path().to_path_buf();

    let mut listener = server::Listener::new("127.0.0.1", "0").await?;
    let target_addr = listener.local_addr()?.to_string();

    tokio::spawn(async move {
        let _ = listener.listen(server_path).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Upload file
    transfer::upload_file(&upload_file, &target_addr).await?;

    // Download file
    let file_name = "test.txt";
    transfer::download_file(Path::new(file_name), &target_addr, &download_path).await?;

    let downloaded = download_path.join(file_name);
    assert!(
        downloaded.exists(),
        "Downloaded file not found at {:?}",
        downloaded
    );

    let content = fs::read_to_string(&downloaded).await?;
    assert_eq!(content, "Hello, test testing!", "File content mismatch");

    Ok(())
}
