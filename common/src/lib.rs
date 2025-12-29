use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
pub mod protocol;
use protocol::ProtocolConnection;
// cli command arg
// PartialEQ for unit test
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Command {
    Upload,   // Upload file
    Download, // Download file
    List,     // Lists the directories from server's resource folder
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FileHeader {
    pub command: Command,
    pub name: String,
    pub size: u64,    // u64 is standard for files
    pub hash: String, // hex string
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_header_serialisation() {
        // set file name
        let file_name: &str = "img.png";

        // instantiate file header
        let original_file_header: FileHeader = FileHeader {
            command: Command::Download,
            name: String::from(file_name),
            size: 4001,
            hash: String::from("abc123def"),
        };
        // serialise to JSON (Struct -> String)
        let json_string_wrapped = serde_json::to_string(&original_file_header);
        // unwrap JSON
        let json_string = json_string_wrapped.unwrap();

        // test if file name is inside of json
        assert!(json_string.contains(file_name));

        // Deserialise (String -> Struct)
        let deserialised_json_wrapped = serde_json::from_str(&json_string);
        let deserialised_json = deserialised_json_wrapped.unwrap();

        assert_eq!(original_file_header, deserialised_json);
    }
    #[tokio::test]
    async fn test_protocol_read_and_write() -> Result<(), Box<dyn std::error::Error>> {
        let stream = TcpStream::connect("127.0.0.1:8080").await?;
        let connection = ProtocolConnection::new(stream).await?;

        // set file name
        let file_name: &str = "img.png";

        // instantiate file header
        let original_file_header: FileHeader = FileHeader {
            command: Command::Download,
            name: String::from(file_name),
            size: 4001,
            hash: String::from("abc123def"),
        };
        // serialise to JSON (Struct -> String)
        let json_string_wrapped = serde_json::to_string(&original_file_header);
        let json_string = json_string_wrapped.unwrap();
        let result = connection.send_header(&json_string).await;
        let header_length = connection.read_prefix().await?;
        let byte_header = connection.read_body(header_length).await?;
        let header = String::from_utf8_lossy(&byte_header);
        assert_eq!(json_string,header);

        Ok(())

    }
}
