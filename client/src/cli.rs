//! CLI Arg Parsing Struct

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// File transfer operations (upload, download, delete, list)
    #[command(group(
  clap::ArgGroup::new("operation")
    .required(true)
    .args(["upload", "download", "delete", "list"]),
  ))]
    Transfer {
        ///  IP of the server (host is added automatically as per config)
        #[arg(short, long)]
        ip: Option<String>,

        /// Upload file to server
        #[arg(short, long, group = "operation")]
        upload: Option<PathBuf>,

        /// Download file from server
        #[arg(short, long, group = "operation")]
        download: Option<PathBuf>,

        /// Delete file from server (full flag required for precaution)
        #[arg(long, group = "operation")]
        delete: Option<PathBuf>,

        /// List all files on server
        #[arg(short, long, group = "operation")]
        list: bool,
    },

    /// Set configuration file values (ip, port, dir)
    Config {
        /// Set new ip
        #[arg(short, long)]
        ip: Option<String>,

        /// Set new port
        #[arg(short, long)]
        port: Option<String>,

        /// Set new download directory
        #[arg(short, long)]
        dir: Option<String>,
    },
}
