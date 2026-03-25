use clap::Parser;

use crate::cli::Args;

mod cli;
mod config;
mod transfer;
mod ui;

// Start tokio engine
#[tokio::main]
async fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    // Load config
    let config = config::ClientConfig::load();

    // See if CLI argument was passed otherwise use config
    let ip = args.ip.unwrap_or_else(|| config.address());

    // Handle CLI arguments

    // Get the result of the function that is called via cli args
    // Use Some operator for Option
    let result = if let Some(path) = args.upload {
        // Upload
        transfer::upload_file(&path, &ip).await
    } else if let Some(path) = args.download {
        // Download
        transfer::download_file(&path, &ip).await
    } else {
        // List
        Ok(())
    };

    // Global Error Handler
    match result {
        // Success
        Ok(_) => {
            println!("Success!");
        }

        // Handle error
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
