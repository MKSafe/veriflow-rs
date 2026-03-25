//! CLI Arg Parsing Struct

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
#[command(group(
  clap::ArgGroup::new("operation")
    .required(true)
    .args(["upload", "download", "delete", "list"]),
))]
pub struct Args {
    ///  IP of the server (host is added automatically)
    // - defaults to localhost
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    pub ip: String,

    /// Upload file to server
    #[arg(short, long, group = "operation")]
    pub upload: Option<PathBuf>,

    /// Download file from server
    #[arg(short, long, group = "operation")]
    pub download: Option<PathBuf>,

    /// Delete file from server (long flag for safety)
    #[arg(long, group = "operation")]
    pub delete: Option<PathBuf>,

    /// List all files on server
    #[arg(short, long, group = "operation")]
    pub list: bool,
}
