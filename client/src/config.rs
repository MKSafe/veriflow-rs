//! Client Config Struct

use serde::Deserialize;

// Config Struct
#[derive(Deserialize, Debug)]
pub struct ClientConfig {
    pub ip: String,
    pub port: String,
    pub download_dir: String,
}

// Skeleton for the config file
impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            ip: String::from("127.0.0.1"),
            port: String::from("8080"),
            download_dir: String::from("../Veriflow/Downloads"),
        }
    }
}