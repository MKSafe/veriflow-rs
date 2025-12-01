mod hashing;

// Start tokio engine
#[tokio::main]
async fn main() {
    // call function to get file
    let path = std::path::Path::new("dummy.txt");

    // get SHA256 of file @path
    let result: Result<String, std::io::Error> = hashing::hash_file(path).await;

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
