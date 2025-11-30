mod hashing;

// Start tokio engine
#[tokio::main]
async fn main() -> std::io::Result<()> {

    // call function to get file
    let path = std::path::Path::new("dummy.txt");
    hashing::hash_file(path).await?;


    // Send back if successful
    Ok(println!("Success!"))
}
