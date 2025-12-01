
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener,TcpStream};
use tracing::{error,info};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::io;
use std::sync::{Arc,Mutex};
pub struct Listener{
    listener: TcpListener,
    open_connections: HashMap<SocketAddr,Arc<Mutex<TcpStream>>>,
    //tasks:
}

impl Listener{
    pub async fn new(host:&str,port:&str,)-> io::Result<Listener>{
        if host == "" || port == ""{
            let listener = TcpListener::bind("127.0.0.1:8080").await?;
            info!("Listener is running");
            return Ok(Listener{listener: listener, open_connections: HashMap::new()})
        }
        let addr = format!("{}:{}",host,port);
        let listener = TcpListener::bind(addr).await?;
        info!("Listener is running");
        return Ok(Listener{listener:listener, open_connections: HashMap::new()})
    }
    /*pub async fn start_listener(&mut self,host: &str, port: &str){

    }*/
    pub async fn listen(&mut self)-> io::Result<()>{
        loop{
            match self.listener.accept().await{
                Ok((mut _stream,addr)) =>{
                    if !self.open_connections.contains_key(&addr){
                        let stream = Arc::new(Mutex::new(_stream));
                        let welcom_msg = format!("Welcome to the Veriflow Resource Server\n User: {}",addr);
                        let write_bytes = _stream.write(welcom_msg.as_bytes()).await?;
                        self.open_connections.insert(addr,stream.clone());
                        info!("User {} has connected. Total: {}",addr,self.open_connections.len());
                    }
                }
                
                Err(e)=>error!("The following error has occured while trying to connect to the client: {}",e)
            }
        }
    }

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

    /*async fn write_to_stream(&mut stream)-> io::Result<u8>{
    }*/

    async fn handle_client(stream: TcpStream,) ->io::Result<bool>{
        tokio::spawn(async move {  
                            if let Err(e) = Listener::read_stream(stream).await{
                                let addr = stream.peer_addr();
                                error!("Error with {addr}:{e}")
                            }
                        });
        return Ok(true);
    }
}