use common::protocol::ProtocolConnection;
use common::Command;
use common::FileHeader;
/*use common::VeriflowError;*/
use sha2::{Digest, Sha256};
use std::io;
use std::path::Path;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};
///This struct represents the listener that will handle connections
pub struct Listener {
    //Struct definition
    listener: TcpListener,
}

impl Listener {
    const BUFFER_SIZE: usize = 4096;
    const FILE_PATH: &str = "../Veriflow/resources/";
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
    /// async fn some_func() -> std::io::Result<()>{
    ///     use server::server::Listener;
    ///     let listener = Listener::new("127.0.0.1","0").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(host: &str, port: &str) -> io::Result<Listener> {
        let path_exists = tokio::fs::try_exists(Self::FILE_PATH).await?;
        if !path_exists {
            fs::create_dir_all(Self::FILE_PATH).await?;
        }
        //When the host or the port is not present run the server on the local host
        if host.is_empty() || port.is_empty() {
            let listener = TcpListener::bind("127.0.0.1:0").await?;
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
    /// async fn some_func() -> std::io::Result<()>{
    ///     use server::server::Listener;
    ///     let mut listener = Listener::new("x.x.x.x","xxxx").await?;
    ///     listener.listen().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn listen(&mut self) -> io::Result<()> {
        //infitnite loop this will act as the servers main loop
        loop {
            //The listener.accept() function can possibly throw an error so we handle it using the match keyword
            match self.listener.accept().await {
                //when a connection is made we deal with it below
                Ok((mut _stream, addr)) => {
                    info!("User {} has connected.", addr,);
                    let connection = ProtocolConnection::new(_stream).await?;
                    tokio::spawn(async move {
                        let _ = Self::handle_client(connection).await;
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
    async fn handle_client(mut connection: ProtocolConnection) -> io::Result<()> {
        let prefix_len = connection.read_prefix().await?;
        let header: Vec<u8> = connection.read_body(prefix_len).await?;
        let string_header = String::from_utf8_lossy(&header);
        let file_header: FileHeader = serde_json::from_str(&string_header).unwrap();
        Self::handle_operation(&file_header, connection).await?;
        Ok(())
    }
    ///Function to manage the client operations
    async fn handle_operation(
        header: &FileHeader,
        connection: ProtocolConnection,
    ) -> io::Result<()> {
        let operation = &header.command;
        match operation {
            Command::Upload => {
                Self::handle_upload(header, connection).await?;
            }
            Command::Download => {
                Self::handle_download(header, connection).await?;
            }
            Command::List => {
                //Self::handle_list(connection).await?;
            }
        }
        Ok(())
    }
    ///Handles clients' upload operation
    async fn handle_upload(
        header: &FileHeader,
        mut connection: ProtocolConnection,
    ) -> io::Result<()> {
        let filename: &String = &header.name;
        let full_file_path = String::from(Self::FILE_PATH) + filename;
        let mut received_file = File::create(&full_file_path).await?;
        connection
            .read_file_to_disk(&mut received_file, header.size)
            .await?;
        let received_file_hash = Self::hash_file(Path::new(&full_file_path)).await?;
        if header.hash != received_file_hash {
            fs::remove_file(full_file_path).await?;
            error!("There has been an error when comparing the expected hash to the calculated hash retry sending the file");
        } else {
            info!("File successfuly received");
        }
        Ok(())
    }
    ///Handles a clients download request
    async fn handle_download(
        header: &FileHeader,
        mut connection: ProtocolConnection,
    ) -> io::Result<()> {
        let filename: String = header.name.clone();
        let full_file_path = String::from(Self::FILE_PATH) + &filename;
        let mut file_to_send = File::open(&full_file_path).await?;
        let file_meta_data = fs::metadata(&full_file_path).await?;
        let file_size = file_meta_data.len();
        let file_hash = Self::hash_file(Path::new(&full_file_path)).await?;
        let file_header = FileHeader {
            command: Command::Upload,
            name: filename,
            size: file_size,
            hash: file_hash,
        };
        let serialized_header_wrapped = serde_json::to_string(&file_header);
        let serialized_header_unwrapped = serialized_header_wrapped.unwrap();
        connection.send_header(&serialized_header_unwrapped).await?;
        connection
            .write_file_to_stream(&mut file_to_send, file_size)
            .await?;
        Ok(())
    }

    /*async fn handle_list(mut connection: ProtocolConnection) -> io::Result<()> {
        //Milo TODO
        Ok(())
    }*/
    ///Accept a single tcp connection
    /// # Returns
    ///
    /// A 'TcpStream' representing the connection
    pub async fn accept_once(&mut self) -> io::Result<TcpStream> {
        //test only method that accepts a single tcp stream
        let (stream, _) = self.listener.accept().await?;
        Ok(stream)
    }
    ///Returns the current address bound to the listener
    pub fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }
    ///Calculates the SHA256 hash of a file
    async fn hash_file(path: &Path) -> io::Result<String> {
        // Buffer
        let mut buffer: [u8; Self::BUFFER_SIZE] = [0; Self::BUFFER_SIZE];

        // get file with tokio
        let mut file = File::open(path).await?;

        // create hasher for SHA256
        let mut hasher: Sha256 = Sha256::new();

        // read file using buffer
        loop {
            // Read chunk from file (number of bytes successfully read)
            let bytes_read: usize = file.read(&mut buffer).await?;

            // finish reading file
            if bytes_read == 0 {
                // break loop
                break;
            }

            // load the chunk from file
            let current_chunk: &[u8] = &buffer[..bytes_read];

            // update hasher with current chunk reference
            hasher.update(current_chunk);
        }

        // finalise hasher, get its output (byte array)
        let file_hash = hasher.finalize();

        // Convert hash (byte array) to hex
        let file_hash_hex: String = format!("{:x}", file_hash);

        // Send back if successful
        Ok(file_hash_hex)
    }
}
