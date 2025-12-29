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

    // Handle CLI arguments
    
    // Get the result of the function that is called via cli args
    // Use Some operator for Option
    let result = if let Some(path) = args.upload {
      // Upload
      transfer::upload_file(&path, &args.ip).await
    } else {
      println!("WIP...");
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
    

    /* 

    // call function to get file
    let file_path = &args.upload.unwrap();

    // get SHA256 of file @path
    let result: Result<String, std::io::Error> = hashing::hash_file(file_path).await;
    


    // Global Error Handler
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
    */
}
