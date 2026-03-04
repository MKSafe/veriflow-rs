use server::server::Listener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    const FILE_PATH: &str = "../Veriflow/resources/";
    let path_exists = tokio::fs::try_exists(FILE_PATH).await?;
    if !path_exists {
        tokio::fs::create_dir_all(FILE_PATH).await?;
    }
    tracing_subscriber::fmt::init();
    let mut listener = Listener::new("0.0.0.0", "8080").await?;
    listener.listen(FILE_PATH.to_string()).await?;
    Ok(())
}
