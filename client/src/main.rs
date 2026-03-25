use clap::Parser;

use crate::cli::Args;

mod cli;
mod transfer;
mod ui;

// Start tokio engine
#[tokio::main]
async fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    // Handle CLI arguments

    // Get the result of the function that is called via cli args
    // Use Some operator for Option
    let result = if let Some(path) = args.upload {
        // Upload
        transfer::upload_file(&path, &args.ip).await
    } else if let Some(path) = args.download {
        // Download
        transfer::download_file(&path, &args.ip).await
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
