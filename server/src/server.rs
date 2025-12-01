
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tracing::info;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::io;
pub struct Listener{
    listener: TcpListener,
    open_connections: HashMap<SocketAddr,TcpStream>,
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
            let (stream,addr) = self.listener.accept().await?;
            if !self.open_connections.contains_key(&addr){
                self.open_connections.insert(addr,stream);
                info!("User {} has connected. Total: {}",addr,self.open_connections.len());
            }
        }
    }
}