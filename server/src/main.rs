use server::server::Listener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let mut listener = Listener::new("0.0.0.0", "8080").await?;
    listener.listen().await?;
    Ok(())
}
