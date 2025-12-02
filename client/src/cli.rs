//! CLI Arg Parsing Struct

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    // Path to file
    pub file_path: PathBuf,
}
