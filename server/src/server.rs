use std::io;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};
pub struct Listener {
    //Struct definition
    listener: TcpListener,
}

impl Listener {
    //implementation of operations for the Listener struct
    pub async fn new(host: &str, port: &str) -> io::Result<Listener> {
        //When the host or the port is not pressent run the server on the local host
        if host.is_empty() || port.is_empty() {
            let listener = TcpListener::bind("127.0.0.1:8080").await?;
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
    /*pub async fn start_listener(&mut self,host: &str, port: &str){

    }*/
    pub async fn listen(&mut self) -> io::Result<()> {
        //infitnite loop this will act as the servers main loop
        loop {
            //The listener.accept() function can possibly throw an error so we handle it using the match keyword
            match self.listener.accept().await {
                //when a connec tion is made we deal with it below

                //when a connection is made we deal with it below
                Ok((mut _stream, addr)) => {
                    info!("User {} has connected.", addr,);
                }

                Err(e) => error!(
                    "The following error has occured while trying to connect to the client: {}",
                    e
                ),
            }
        }
    }
    pub async fn accept_once(&mut self) -> io::Result<TcpStream> {
        //test only method that accepts a single tcp stream
        let (stream, _) = self.listener.accept().await?;
        Ok(stream)
    }
    pub fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        //returns the address the server is bound to
        self.listener.local_addr()
    }
    /*
        async fn read_stream(stream: TcpStream,) -> io::Result<String>{
            let mut buffer = vec![0u8;4096];
            let mut msg = String::new();
            loop{
                stream.readable().await?;
                match stream.try_read(&mut buffer){
                    Ok(0)=>{info!("Client finished sending");break;}
                    Ok(n)=>{
                        msg.push_str(&String::from_utf8_lossy(&buffer[..n]));
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
            Ok(msg)
        }
    async fn write_to_stream(&mut stream)-> io::Result<u8>{
        }

    async fn handle_client(stream: TcpStream, addr: SocketAddr) -> io::Result<bool> {
        Ok(true)
    }
    */
}
