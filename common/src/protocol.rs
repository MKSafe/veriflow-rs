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
    
    pub async fn send_header(&self, header: FileHeader)->io::Result<bool>{
        //turns data into bytes and gets the length0.
        let data_as_bytes = header.to_bytes();
        let data_byte_len = data_as_bytes.len() as u32;
        //initialises a buffer with the size of the data length prefix and data length
        let mut buffer = Vec::with_capacity(4+data_as_bytes.len());
        //adds the len prefix to the buffer
        buffer.extend_from_slice(&data_byte_len.to_be_bytes());
        //adds the data to the byte buffer
        buffer.extend_from_slice(&data_as_bytes);

        loop{
            //checks the streams state for writability 
            match self.stream.ready(Interest::WRITEABLE).await{
                Ok(state)=>{
                    if state.is_writable(){
                        //if the state is writeable send the packet
                        match self.stream.try_write.await{
                            Ok(n)=>{
                                //check if the whole packet has been sent
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
                        return Ok(false);
                    }
                }
                Err(e)=>{
                    //if there is an error with retriving the socket state print the error to console 
                    error!("The following error has occured: {}",e)
                    return Ok(false);
                }
            };
        }
    }

   /* pub async fn send_file(&self,file : Vec<u8>)-> io::Result<bool>{
        loop{
            //todo
        
        }
    }*/

    pub async fn read(&self,buffer_len : Option<u8>)-> io::Result<Vec<u8>>{
        let mut len_buff:Option<Vec<u8>> = None;
        let mut body_buff:Option<Vec<u8>> = None;
        if buffer_len.is_none() {
            len_buff = Some([Vec<u8>::with_capacity(4)]);
        }
        if len_buff.is_some(){

        }
        match self.stream.ready(Interest::READABLE).await{
            Ok(state)=>{
                if state.is_readable(){
                    self.stream.try_read()
                }else if state.is_read_closed(){
                
                }

            }
            Err(e)=>{
                error!("An error has occured trying to read data: {}",e);
                return Ok([0]);
            }
        }
        Ok(Vec<u8>::new())
    }
}