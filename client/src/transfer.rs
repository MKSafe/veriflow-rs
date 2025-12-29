//! File Upload Logic

use std::{path::Path, fs};
use tokio::fs::File;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use common::{FileHeader, Command, VeriflowError};
use crate::{hashing, ui};

