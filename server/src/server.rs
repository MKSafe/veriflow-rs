use common::hashing;
use common::protocol::ProtocolConnection;
use common::Command;
use common::FileHeader;
use common::VeriflowError;
use std::io;
use std::path;
use std::path::{Component, Path, PathBuf};
use tokio::fs;
use tokio::fs::metadata;
use tokio::fs::File;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};
///This struct represents the listener that will handle connections
pub struct Listener {
    //Struct definition
    listener: TcpListener,
}

impl Listener {
    ///Used to initialise a new server listener
    /// # Arguments
    /// * 'host' - A '&str' which represents the ip address of the server
    /// * 'port' - A '&str' which represents the port our server is going to listen on
    ///
    /// # Returns
    /// A 'Result' containing the 'Listener' struct object which will listen for client connections
    ///
    /// #Examples
    /// ```
    /// async fn some_func() -> common::Result<()> {
    ///     use server::server::Listener;
    ///     let listener = Listener::new("127.0.0.1","0").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(host: &str, port: &str) -> common::Result<Listener> {
        //When the host or the port is not present run the server on the local host
        if host.is_empty() || port.is_empty() {
            let listener = TcpListener::bind("0.0.0.0:8080").await?;
            let port = listener.local_addr().unwrap().port();
            info!("Listener is running on {}", port);
            //returns a new listener struct object
            return Ok(Listener { listener });
        }
        //If the host and port is specified the server will be ran with the passed address
        let addr = format!("{}:{}", host, port);
        let listener = TcpListener::bind(&addr).await?;
        info!("Listener is running {}", addr);
        //returns a new listener struct
        Ok(Listener { listener })
    }
    ///This starts the server loop which accepts a connection and handles the client
    ///
    /// #Examples
    /// ```
    /// async fn some_func() -> common::Result<()> {
    ///     use server::server::Listener;
    ///     let mut listener = Listener::new("x.x.x.x","xxxx").await?;
    ///     listener.listen("directory of the resources you want to share".to_string().into()).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn listen(&mut self, path: PathBuf) -> common::Result<()> {
        //infitnite loop this will act as the servers main loop
        loop {
            //The listener.accept() function can possibly throw an error so we handle it using the match keyword
            match self.listener.accept().await {
                //when a connection is made we deal with it below
                Ok((mut _stream, addr)) => {
                    info!("User {} has connected.", addr,);
                    let connection = ProtocolConnection::new(_stream).await?;
                    let dir = path.clone();
                    tokio::spawn(async move {
                        let _ = Self::handle_client(connection, dir).await;
                    });
                }

                Err(e) => error!(
                    "The following error has occured while trying to connect to the client: {}",
                    e
                ),
            }
        }
    }
    ///Used to concurrently handle clients
    async fn handle_client(
        mut connection: ProtocolConnection,
        path: PathBuf,
    ) -> common::Result<()> {
        let prefix_len = connection.read_prefix().await?;
        let header: Vec<u8> = connection.read_body(prefix_len).await?;
        let string_header = String::from_utf8_lossy(&header);
        let file_header: FileHeader = serde_json::from_str(&string_header)?;
        Self::handle_operation(&file_header, connection, path).await?;
        Ok(())
    }
    async fn safe_join(base: &Path, user_input: &str) -> common::Result<path::PathBuf> {
        let path = Path::new(user_input);
        if user_input.is_empty() {
            return Ok(base.to_path_buf());
        }
        if path.is_absolute() {
            return Err(VeriflowError::Io(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Absolute path not allowed",
            )));
        }
        for comp in path.components() {
            if matches!(comp, Component::ParentDir) {
                return Err(VeriflowError::Io(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "Path traversal detected",
                )));
            }
        }
        Ok(base.join(path))
    }
    ///Function to manage the client operations
    async fn handle_operation(
        header: &FileHeader,
        connection: ProtocolConnection,
        path: PathBuf,
    ) -> common::Result<()> {
        let operation = &header.command;
        let path_var = path.as_path();
        let safe_path = Self::safe_join(path_var, &header.name).await?;
        match operation {
            Command::Upload => {
                Self::handle_upload(header, connection, safe_path).await?;
            }
            Command::Download => {
                Self::handle_download(header, connection, safe_path).await?;
            }
            Command::List => {
                Self::handle_list(connection, safe_path).await?;
            }
            Command::Delete => {
                Self::handle_delete(connection, safe_path).await?;
            }
        }
        Ok(())
    }
    ///Handles clients' upload operation
    async fn handle_upload(
        header: &FileHeader,
        mut connection: ProtocolConnection,
        path: PathBuf,
    ) -> common::Result<()> {
        let mut received_file = File::create(&path.as_path()).await?;
        connection
            .read_file_to_disk(&mut received_file, header.size)
            .await?;
        let received_file_hash = hashing::hash_file(path.as_path(), |_| {}).await?;
        if header.hash != received_file_hash {
            fs::remove_file(path).await?;
            error!("There has been an error when comparing the expected hash to the calculated hash retry sending the file");
            let header = FileHeader {
                command: Command::Upload,
                name: "Failure hash does not match".to_string(),
                size: 0,
                hash: ' '.to_string(),
            };
            let str_header = serde_json::to_string(&header)?;
            connection.send_header(&str_header).await?;
        } else {
            info!("File successfuly received");
            let header = FileHeader {
                command: Command::Delete,
                name: "Success".to_string(),
                size: 0,
                hash: ' '.to_string(),
            };
            let str_header = serde_json::to_string(&header)?;
            connection.send_header(&str_header).await?;
        }
        Ok(())
    }
    ///Handles a clients' download request
    async fn handle_download(
        header: &FileHeader,
        mut connection: ProtocolConnection,
        path: PathBuf,
    ) -> common::Result<()> {
        let filename: String = header.name.clone();
        let mut file_to_send = File::open(&path.as_path()).await?;
        let file_meta_data = fs::metadata(&path.as_path()).await?;
        let file_size = file_meta_data.len();
        let file_hash = hashing::hash_file(path.as_path(), |_| {}).await?; // use saved .sha256 sidecar file in future
        let file_header = FileHeader {
            command: Command::Upload,
            name: filename,
            size: file_size,
            hash: file_hash,
        };
        let serialized_header = serde_json::to_string(&file_header)?;
        connection.send_header(&serialized_header).await?;
        connection
            .write_file_to_stream(&mut file_to_send, file_size)
            .await?;
        Ok(())
    }

    ///Handles a list command request
    ///
    /// No return but it walks the resource directory and sends its contents together with the subdirectories to the client
    async fn handle_list(mut connection: ProtocolConnection, path: PathBuf) -> common::Result<()> {
        let mut stack = vec![path.clone()];
        let mut path_list = vec![];
        while let Some(dir) = stack.pop() {
            let mut dir_content = fs::read_dir(dir.clone()).await?;
            while let Some(entry) = dir_content.next_entry().await? {
                let file_type = entry.file_type().await?;
                let entry_path = entry.path();

                if file_type.is_file() {
                    let relative = entry_path.strip_prefix(&path).unwrap_or(&entry_path);

                    let str_path = relative.to_string_lossy().replace("\\", "/");
                    path_list.push(str_path);
                } else if file_type.is_dir() {
                    stack.push(entry_path);
                }
            }
        }
        info!("{:?}", path_list);
        let payload = serde_json::to_vec(&path_list)?;
        let payload_header = FileHeader {
            command: Command::List,
            name: "list".to_string(),
            size: payload.len() as u64,
            hash: ' '.to_string(),
        };
        let str_header = serde_json::to_string(&payload_header)?;
        connection.send_header(&str_header).await?;
        connection.send_data(&payload).await?;
        Ok(())
    }
    ///Handles a delete request
    pub async fn handle_delete(
        mut connection: ProtocolConnection,
        path: PathBuf,
    ) -> common::Result<()> {
        info!("{:?}", &path);
        let md = metadata(&path).await?;
        if md.is_dir() {
            match fs::remove_dir_all(path.as_path()).await {
                Ok(()) => {
                    let header = FileHeader {
                        command: Command::Delete,
                        name: "Success".to_string(),
                        size: 0,
                        hash: ' '.to_string(),
                    };
                    let str_header = serde_json::to_string(&header)?;
                    connection.send_header(&str_header).await?;
                }
                Err(e) => {
                    let header = FileHeader {
                        command: Command::Delete,
                        name: format!("Failed with error {e}").to_string(),
                        size: 0,
                        hash: ' '.to_string(),
                    };
                    let str_header = serde_json::to_string(&header)?;
                    connection.send_header(&str_header).await?;
                }
            }
        } else if md.is_file() {
            match fs::remove_file(path.as_path()).await {
                Ok(()) => {
                    let header = FileHeader {
                        command: Command::Delete,
                        name: "Success".to_string(),
                        size: 0,
                        hash: ' '.to_string(),
                    };
                    let str_header = serde_json::to_string(&header)?;
                    connection.send_header(&str_header).await?;
                }
                Err(e) => {
                    let header = FileHeader {
                        command: Command::Delete,
                        name: format!("Failed with error {e}").to_string(),
                        size: 0,
                        hash: ' '.to_string(),
                    };
                    let str_header = serde_json::to_string(&header)?;
                    connection.send_header(&str_header).await?;
                }
            }
        }

        Ok(())
    }
    ///Accept a single tcp connection
    /// # Returns
    ///
    /// A 'TcpStream' representing the connection
    pub async fn accept_once(&mut self) -> common::Result<TcpStream> {
        //test only method that accepts a single tcp stream
        let (stream, _) = self.listener.accept().await?;
        Ok(stream)
    }
    ///Returns the current address bound to the listener
    pub fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }
}
