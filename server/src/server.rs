use std::fs::File;
use std::io;
use common::VeriflowError;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};
use common::protocol::ProtocolConnection;
use common::FileHeader;
use common::Command;
<<<<<<< HEAD
use serde;
=======
>>>>>>> 1e5ecc399f1d1a4ab78e932246126be50af091d6
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
    /// async fn some_func() -> std::io::Result<()>{
    ///     use server::server::Listener;
    ///     let listener = Listener::new("127.0.0.1","0").await?;
    ///     Ok(())c
    /// }
    /// ```
    pub async fn new(host: &str, port: &str) -> io::Result<Listener> {
        //When the host or the port is not pressent run the server on the local host
        if host.is_empty() || port.is_empty() {
            let listener = TcpListener::bind("127.0.0.1:0").await?;
            info!("Listener is running");
            //returns a new listener struct object
            return Ok(Listener { listener });
        }
        //If the host and port is specified the server will be ran with the passed address
        let addr = format!("{}:{}", host, port);
        let listener = TcpListener::bind(addr).await?;
        info!("Listener is running");
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
                    let mut connection = ProtocolConnection::new(_stream).await?;
                    let client_task: tokio::task::JoinHandle<Result<(),Box<VeriflowError>>>  = tokio::spawn(async move{
                        self.handle_client(&mut connection).await?;
                        Ok(())
                    });
                }

                Err(e) => error!(
                    "The following error has occured while trying to connect to the client: {}",
                    e
                ),
            }
        }
    }
<<<<<<< HEAD
    pub async fn handle_client(&mut self, connection : &mut ProtocolConnection) -> io::Result<()>{
        let prefix_len = connection.read_prefix().await?;
        let header = connection.read_body(prefix_len).await?;
        let string_header = String::from_utf8_lossy(&header);
        let file_header : FileHeader = serde_json::from_str(&string_header).unwrap();
        self.handle_operation(&file_header).await?;
        Ok(())
    }

    async fn handle_operation(&mut self, header : &FileHeader) -> io::Result<()>{
        let operation = header.command;
        match operation {
            Command::Upload => {

            }
            Command::Download => {

            }
            Command::List => {

            }
            _ => {
                Err()
            }
        }
        Ok(())
=======
    pub async fn handle_client(&mut self, connection : &mut ProtocolConnection){

>>>>>>> 1e5ecc399f1d1a4ab78e932246126be50af091d6
    }
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
}
