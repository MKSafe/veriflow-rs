use tokio::fs::File;
use std::path::Path;
use std::io;

pub async fn hash_file(path: &Path) -> io::Result<()> {
  // get file with tokio
  let mut file = File::open(path).await?;


  // Send back if successful
  Ok(())  
}