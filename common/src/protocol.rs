use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::error;
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
    /// async fn some_func() -> std::io::Result<()> {
    ///     use common::protocol::ProtocolConnection;
    ///     use tokio::net::TcpStream;
    ///     let stream = TcpStream::connect("127.0.0.1").await?;
    ///     let connection = ProtocolConnection::new(stream).await?;
    ///     Ok(())
    /// }
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
    pub async fn send_header(&mut self, header: &str) -> io::Result<bool> {
        //turns data into bytes and gets the length
        let data_as_bytes = header.as_bytes();
        let data_byte_len = data_as_bytes.len() as u32;
        //initialises a buffer with the size of the data length prefix and data length
        let mut buffer = Vec::with_capacity(data_as_bytes.len());
        //adds the len prefix to the buffer
        let mut len_buff = Vec::with_capacity(data_byte_len.to_be_bytes().len());
        len_buff.extend_from_slice(&data_byte_len.to_be_bytes());
        //adds the data to the byte buffer
        buffer.extend_from_slice(data_as_bytes);
        match self.send_file(&mut len_buff).await {
            Ok(true) => match self.send_file(&mut buffer).await {
                Ok(true) => Ok(true),
                Ok(false) => Ok(false),
                Err(e) => {
                    error!("Following error occured: {}", e);
                    Ok(false)
                }
            },
            Ok(false) => {
                error!("Failed sending the header prefix length");
                Ok(false)
            }
            Err(e) => {
                error!("Following error occured: {}", e);
                Ok(false)
            }
        }
    }
    /// Sending function designed to send data based on a buffer
    /// # Arguments
    /// * 'buffer' - a '&mut Vec<u8>' which contains the data to be sent in byte format
    ///  
    /// # Returns
    /// A 'Result' containing 'bool' which represents the success of the send functions
    pub async fn send_file(&mut self, buffer: &mut Vec<u8>) -> io::Result<bool> {
        match self.stream.write_all(buffer).await {
            Ok(()) => Ok(true),
            Err(e) => {
                error!("The following error has occured {}", e);
                Ok(false)
            }
        }
    }

    ///Reads the prefixed length of the header
    ///
    /// #Returns
    /// A 'Result' of usize representing the size of the incoming header
    pub async fn read_prefix(&mut self) -> io::Result<usize> {
        //creates the prefix buffer
        let mut buf: [u8; 4] = [0u8; 4];
        match self.stream.read_exact(&mut buf).await {
            Ok(n) => {
                if n != buf.len() {
                    error!("Failed to read the whole prefix");
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "invalid length read",
                    ));
                } else {
                    let value = u32::from_be_bytes(buf) as usize;
                    Ok(value)
                }
            }
            Err(e) => {
                error!("Following error occured: {}", e);
                Err(e)
            }
        }
    }
    ///Reads a number of bytes specified
    /// # Arguments
    /// * 'buffer_len' - Represents the number of bytes to read
    ///
    /// # Returns
    /// A 'Result' of 'Vec<u8>' which is the data received through the connection stream
    pub async fn read_body(&mut self, buffer_len: usize) -> io::Result<Vec<u8>> {
        //creates a buffer for a custom size
        let mut buf = vec![0u8; buffer_len];
        match self.stream.read_exact(&mut buf).await {
            Ok(n) => {
                if n != buffer_len {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "invalid length read",
                    ));
                } else {
                    Ok(buf)
                }
            }
            Err(e) => {
                error!("Following error occured: {}", e);
                Err(e)
            }
        }
    }
}
