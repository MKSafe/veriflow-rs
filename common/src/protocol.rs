use std::io;
use tokio::io::Interest;
use tokio::net::TcpStream;
use tracing::{error, info};
///Represents the custom Protocol read and send methods built on top of Tcp
pub struct ProtocolConnection {
    stream: TcpStream,
}

impl ProtocolConnection {
    /// Creates a new protocol connection
    ///
    /// # Arguments
    /// * 'stream' - takes in a 'TcpStream' to base our protocol connection on
    ///
    /// # Returns
    /// A new custom protocol connection
    /// # Examples
    ///
    /// ```
    /// use protocol::ProtocolConnection;
    /// use tokio::net::TcpStream;
    /// let stream = TcpStream::connect("x.x.x.x","xxxx").await?;
    /// let connection = ProtocolConnection::new(stream).await?;
    /// ```
    pub async fn new(stream: TcpStream) -> io::Result<ProtocolConnection> {
        //returns a new connection object
        Ok(ProtocolConnection { stream })
    }
    /// Sends the custom json header
    ///
    /// # Arguments
    /// * 'header' - A '&str' that contains the serialized josn
    ///
    /// # Returns
    /// A 'bool' value to represent if the send function succeeded
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

        match self.send_file(&mut buffer).await {
            Ok(true) => Ok(true),
            Ok(false) => Ok(false),
            Err(e) => {
                error!("Following error occured {}", e);
                Err(e)
            }
        }
    }
    /// Sending function designed to send data based on a buffer
    /// # Arguments
    /// * 'buffer' - a '&mut Vec<u8>' which contains the data to be sent in byte format
    ///  
    /// # Returns
    /// A 'Result' containing 'bool' which represents the success of the send functions
    pub async fn send_file(&self, buffer: &mut Vec<u8>) -> io::Result<bool> {
        loop {
            //checks if the connected stream is active for a write action
            match self.stream.ready(Interest::WRITABLE).await {
                Ok(state) => {
                    if state.is_writable() {
                        //when writable the data will be sent
                        match self.stream.try_write(buffer) {
                            Ok(n) => {
                                if n == buffer.len() {
                                    return Ok(true);
                                } else {
                                    buffer.drain(..n);
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

    ///Reads the prefixed length of the header
    ///
    /// #Returns
    /// A 'Result' of usize representing the size of the incoming header
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
    ///Reads a number of bytes specified
    /// # Arguments
    /// * 'buffer_len' - Represents the number of bytes to read
    ///
    /// # Returns
    /// A 'Result' of 'Vec<u8>' which is the data received through the connection stream
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
