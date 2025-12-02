use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};
pub struct Listener {
    //Struct definition
    listener: TcpListener,
    open_connections: HashMap<SocketAddr, TcpStream>,
}

impl Listener {
    //implementation of operations for the Listener struct
    pub async fn new(host: &str, port: &str) -> io::Result<Listener> {
        //When the host or the port is not pressent run the server on the local host
        if host.is_empty() || port.is_empty() {
            let listener = TcpListener::bind("127.0.0.1:8080").await?;
            info!("Listener is running");
            let open_connections: HashMap<SocketAddr, TcpStream> = HashMap::new();
            //returns a new listener struct object
            return Ok(Listener {
                listener,
                open_connections: open_connections,
            });
        }
        //If the host and port is specified the server will be ran with the passed address
        let addr = format!("{}:{}", host, port);
        let listener = TcpListener::bind(addr).await?;
        info!("Listener is running");
        //returns a new listener struct
        let open_connections: HashMap<SocketAddr, TcpStream> = HashMap::new();
        return Ok(Listener {
            listener,
            open_connections: open_connections,
        });
    }
    /*pub async fn start_listener(&mut self,host: &str, port: &str){

    }*/
    pub async fn listen(&mut self) -> io::Result<()> {
        //infitnite loop this will act as the servers main loop
        loop {
            //The listener.accept() function can possibly throw an error so we handle it using the match keyword
            match self.listener.accept().await {
                //when a connec tion is made we deal with it below
                Ok((mut _stream, addr)) => {
                    if !self.open_connections.contains_key(&addr) {
                        //if the current address is already connected
                        //let welcom_msg = format!("Welcome to the Veriflow Resource Server\n User: {}",addr);
                        //let write_bytes = _stream.write(welcom_msg.as_bytes()).await?;
                        self.open_connections.insert(addr, _stream);
                        info!(
                            "User {} has connected. Total: {}",
                            addr,
                            self.open_connections.len()
                        );
                    }
                }

                Err(e) => error!(
                    "The following error has occured while trying to connect to the client: {}",
                    e
                ),
            }
        }
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

        async fn handle_client(stream: TcpStream,) ->io::Result<bool>{
            tokio::spawn(async move {
                          if let Err(e) = Listener::read_stream(stream).await{
                                    let addr = stream.peer_addr();
                                    error!("Error with {addr}:{e}")
                                }
                            });
            return Ok(true);
        }*/
}
