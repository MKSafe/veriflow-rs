use std::io;
use tokio::net::TcpStream;

struct ProtocolConnection{
    stream: TcpStream,
}

impl ProtocolConnection{
    pub async fn new(_stream : TcpStream)-> io::Result<ProtocolConnection>{
        Ok(ProtocolConnection{_stream})
    }

    pub async fn send(&self)-> io::Result<Vec<u8>>{
        //TODO
        Ok(Vec<u8>::new())
    }

    pub async fn read(&self)-> io::Result<Vec<u8>>{
        //TODO
        Ok(Vec<u8>::new())
    }
}