use crate::FileHeader;
use std::io;
use tokio::io::Interest;
use tokio::io::Ready;
use tokio::net::TcpStream;
use tracing::{error, info};
pub struct ProtocolConnection {
    stream: TcpStream,
}

impl ProtocolConnection {
    pub async fn new(stream: TcpStream) -> io::Result<ProtocolConnection> {
        //returns a new connection object
        Ok(ProtocolConnection { stream })
    }

    pub async fn send_header(&self, header: &str) -> io::Result<bool> {
        //turns data into bytes and gets the length
        let data_as_bytes = header.as_bytes();
        let data_byte_len = data_as_bytes.len() as u32;
        //initialises a buffer with the size of the data length prefix and data length
        let mut buffer = Vec::with_capacity(4 + data_as_bytes.len());
        //adds the len prefix to the buffer
        buffer.extend_from_slice(&data_byte_len.to_be_bytes());
        //adds the data to the byte buffer
        buffer.extend_from_slice(&data_as_bytes);

        loop {
            //checks the streams state for writability
            match self.stream.ready(Interest::WRITABLE).await {
                Ok(state) => {
                    if state.is_writable() {
                        //if the state is writeable send the packet
                        match self.stream.try_write(&buffer) {
                            Ok(n) => {
                                //check if the whole packet has been sent
                                if n == buffer.len() {
                                    return Ok(true);
                                } else {
                                    buffer.drain(..n);
                                }
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                continue;
                            }
                            Err(e) => {
                                return Err(e.into());
                            }
                        }
                    } else if state.is_write_closed() {
                        info!("The socket you are trying to write to is not accessible");
                        return Ok(false);
                    }
                }
                Err(e ) => {
                    //if there is an error with retriving the socket state print the error to console
                    error!("The following error has occured: {}", e);
                    return Ok(false);
                }
            };
        }
    }

    pub async fn send_file(&self,file : &mut Vec<u8>)-> io::Result<bool>{
        loop{
            //todo
            match self.stream.ready(Interest::WRITABLE).await {
                Ok(state) =>{
                    if state.is_writable(){
                        match self.stream.try_write(&file){
                            Ok(n)=>{
                                if n == file.len(){
                                    return Ok(true);
                                }
                                else{
                                    file.drain(..n);
                                }
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock =>{
                                continue;
                            }
                            Err(e)=>{
                                return Err(e.into());
                            }
                        }
                    }
                    else if state.is_write_closed(){
                        info!("The socket you are trying to write to is not accessible");
                        return Ok(false);
                    }
                }
                Err(e)=>{
                    error!("The following error has occured: {}",e);
                    return Ok(false);
                }
            }
        }
    }

    pub async fn read_prefix(&self) -> io::Result<usize> {
        let mut buf: [u8; 4] = [0u8; 4];
        let mut read: usize = 0;
        while read < 4 {
            match self.stream.ready(Interest::READABLE).await {
                Ok(state) => {
                    if state.is_readable() {
                        match self.stream.try_read(&mut buf[read..]) {
                            Ok(0) => {
                                info!("User disconnected");
                                break;
                            }
                            Ok(n) => {
                                read += n;
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                continue;
                            }
                            Err(e) => {
                                error!("The following error has occured {}", e);
                            }
                        }
                    } else if state.is_read_closed() {
                        error!("The stream is closed for the read operation.")
                    }
                }
                Err(e) => {
                    error!("An error has occured trying to read data: {}", e);
                    return Ok(0);
                }
            }
        }
        Ok(u32::from_be_bytes(buf) as usize)
    }

    pub async fn read_body(&self, buffer_len: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; buffer_len];
        let mut read = 0;
        while read < buffer_len {
            match self.stream.ready(Interest::READABLE).await {
                Ok(state) => {
                    if state.is_readable() {
                        match self.stream.try_read(&mut buf[read..]) {
                            Ok(0) => {
                                info!("User disconnected");
                                break;
                            }
                            Ok(n) => {
                                read += n;
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                continue;
                            }
                            Err(e) => {
                                error!("The following error has occured {}", e);
                            }
                        }
                    } else if state.is_read_closed() {
                        error!("The stream is closed for the read operation.")
                    }
                }
                Err(e) => {
                    error!("An error has occured trying to read data: {}", e);
                    return Ok(vec![0]);
                }
            }
        }
        Ok(buf)
    }
}
