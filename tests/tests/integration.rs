use client::transfer;
use common::VeriflowError;
use server::server;

// External dependencies for tests
use tempfile::tempdir;
use tokio::fs;

#[tokio::test]
async fn integration_test_list_command() -> Result<(), VeriflowError> {
    // setup temp dir
    let temp_dir = tempdir()?;
    let resource_path = temp_dir.path().to_path_buf();

    // Create fake file
    fs::File::create(resource_path.join("some_file.txt")).await?;

    // Initialise server
    let mut listener = server::Listener::new("127.0.0.1", "0").await?;
    let target_addr = listener.local_addr()?.to_string();

    // spawn the server
    tokio::spawn(async move {
        let _ = listener.listen(resource_path).await;
    });

    // Wait until the server starts listening
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    // Send list command
    transfer::list_files(&target_addr).await?;

    Ok(())
}
