use std::io;
use tokio::net::TcpStream;
use tokio::io::Interest;
use tokio::io::Ready;
use tracing::{error, info};
struct ProtocolConnection{
    stream: TcpStream,
}

impl ProtocolConnection{
    pub async fn new(_stream : TcpStream)-> io::Result<ProtocolConnection>{
        //returns a new connection object
        Ok(ProtocolConnection{_stream})
    }

    pub async fn send<T>(&self,data : T)-> io::Result<bool>{
        //turns data into bytes and gets the length
        let data_as_bytes = data.to_bytes();
        let data_byte_len = data_as_bytes.len() as u32;
        //initialises a buffer with the size of the data length prefix and data length
        let mut buffer = Vec::with_capacity(4+data_as_bytes.len());
        //adds the len prefix to the buffer
        buffer.extend_from_slice(&data_byte_len.to_be_bytes());
        //adds the data to the byte buffer
        buffer.extend_from_slice(&data_as_bytes);

        loop{
            match self.stream.ready(Interest::WRITEABLE).await{
                Ok(state)=>{
                    if state.is_writable(){
                        match self.stream.try_write.await{
                            Ok(n)=>{
                                if n == packet.len(){
                                    return Ok(true);
                                }else{
                                    packet.drain(..n);
                                }
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                continue;
                            }
                            Err(e) =>{
                                return Err(e.into());
                            }
                        }

                    }else if state.is_write_closed(){
                        info!("The socket you are trying to write to is not accessible");
                        break;
                    }
                }
                Err(e)=>{
                    error!("The following error has occured: {}",e)
                }
            };
        }
    }

    pub async fn read(&self)-> io::Result<Vec<u8>>{
        //TODO
        Ok(Vec<u8>::new())
    }
}