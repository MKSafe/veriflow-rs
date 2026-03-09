use server::{server::Listener, Config};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[tokio::main]
async fn main() -> std::io::Result<()> {
    const FILE_PATH: &str = "../Veriflow/resources/";
    const CONFIG_PATH: &str = "./config.toml";
    let config_exists = tokio::fs::try_exists(CONFIG_PATH).await?;
    if !config_exists {
        let path_exists = tokio::fs::try_exists(FILE_PATH).await?;
        if !path_exists {
            tokio::fs::create_dir_all(FILE_PATH).await?;
        }
        let config_content: Config = toml::from_str(
            r#"
        [network]
        ip = '127.0.0.1'
        port = '0'

        [directory]
        path = '../Veriflow/resources/'
        "#,
        )
        .unwrap();
        let _ = tokio::fs::File::create(CONFIG_PATH).await?;
        let mut config_file = tokio::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(CONFIG_PATH)
            .await?;
        let string_content = toml::to_string(&config_content).unwrap();
        config_file.write_all(string_content.as_bytes()).await?;
        config_file.flush().await?;
    }
    let mut config_file = tokio::fs::File::open(CONFIG_PATH).await?;
    let mut content = String::new();
    config_file.read_to_string(&mut content).await?;
    let config_struct: Config = toml::from_str(&content).unwrap();
    let path_exists = tokio::fs::try_exists(&config_struct.directory.path).await?;
    if !path_exists {
        tokio::fs::create_dir_all(&config_struct.directory.path).await?;
    }
    tracing_subscriber::fmt::init();
    let mut listener =
        Listener::new(&config_struct.network.ip, &config_struct.network.port).await?;
    listener.listen(config_struct.directory.path).await?;
    Ok(())
}
