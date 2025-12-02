mod server;
use server::Listener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let mut listener = Listener::new("127.0.0.1", "8080").await?;
    listener.listen().await?;
    Ok(())
}
