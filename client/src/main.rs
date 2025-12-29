use clap::Parser;

use crate::cli::Args;

mod cli;
mod hashing;
mod ui;
mod transfer;

// Start tokio engine
#[tokio::main]
async fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    // call function to get file
    let file_path = &args.upload.unwrap();

    // get SHA256 of file @path
    let result: Result<String, std::io::Error> = hashing::hash_file(file_path).await;

    // Handle result
    match result {
        // Success
        Ok(hash) => {
            println!("Success!\nSHA256: {hash}");
        }

        // Handle error
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
