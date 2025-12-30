use std::io;
use tokio::io::Interest;
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
        buffer.extend_from_slice(data_as_bytes);

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
                                error!("the following error occured {}", e);
                                return Ok(false);
                            }
                        }
                    } else if state.is_write_closed() {
                        info!("The socket you are trying to write to is not accessible");
                        return Ok(false);
                    }
                }
                Err(e) => {
                    //if there is an error with retriving the socket state print the error to console
                    error!("The following error has occured: {}", e);
                    return Ok(false);
                }
            };
        }
    }

    pub async fn send_file(&self, file: &mut Vec<u8>) -> io::Result<bool> {
        loop {
            //checks if the connected stream is active for a write action
            match self.stream.ready(Interest::WRITABLE).await {
                Ok(state) => {
                    if state.is_writable() {
                        //when writable the data will be sent
                        match self.stream.try_write(file) {
                            Ok(n) => {
                                if n == file.len() {
                                    return Ok(true);
                                } else {
                                    file.drain(..n);
                                }
                            }
                            //if a blocking error occurs try again
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                continue;
                            }
                            //on error quit the loop and send the error
                            Err(e) => {
                                error!("The following error occured {}", e);
                                return Ok(false);
                            }
                        }
                    //when the socket is unavailable it will display a message and return false
                    } else if state.is_write_closed() {
                        info!("The socket you are trying to write to is not accessible");
                        return Ok(false);
                    }
                }
                //when there is an issue with checking the intention the error is displayed and false is returned
                Err(e) => {
                    error!("The following error has occured: {}", e);
                    return Ok(false);
                }
            }
        }
    }

    pub async fn read_prefix(&self) -> io::Result<usize> {
        //creates the prefix buffer
        let mut buf: [u8; 4] = [0u8; 4];
        //create a counter
        let mut read: usize = 0;
        //makes sure only enough bytes to fill the buffer counter are read
        while read < 4 {
            //checks if the stream is readable
            match self.stream.ready(Interest::READABLE).await {
                Ok(state) => {
                    if state.is_readable() {
                        //tries to read the bytes received into a buffer
                        match self.stream.try_read(&mut buf[read..]) {
                            //when nothing is read print a message and exit the loop
                            Ok(0) => {
                                info!("User disconnected");
                                break;
                            }
                            //if bytes are being read increment read
                            Ok(n) => {
                                read += n;
                            }
                            //if the operation would block then repeat
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
        //creates a buffer for a custom size
        let mut buf = vec![0u8; buffer_len];
        //index for the number of bytes being read
        let mut read = 0;
        while read < buffer_len {
            //checks if current connection is readable
            match self.stream.ready(Interest::READABLE).await {
                Ok(state) => {
                    if state.is_readable() {
                        //attempts to read n amount of bytes
                        match self.stream.try_read(&mut buf[read..]) {
                            //no bytes received then leave the loop
                            Ok(0) => {
                                info!("User disconnected");
                                break;
                            }
                            //increments the read index by the number of bytes received
                            Ok(n) => {
                                read += n;
                            }
                            //if the operation would block then repeat trying to read
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                continue;
                            }
                            //displays and error if for some reason the action cannot be done
                            Err(e) => {
                                error!("The following error has occured {}", e);
                            }
                        }
                    } else if state.is_read_closed() {
                        error!("The stream is closed for the read operation.")
                    }
                }
                //if the intention cannot be received for some reason then displays the error and return nothing
                Err(e) => {
                    error!("An error has occured trying to read data: {}", e);
                    return Ok(vec![0]);
                }
            }
        }
        Ok(buf)
    }
}
