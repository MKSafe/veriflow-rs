use serde::{Deserialize, Serialize};
pub mod protocol;
pub use protocol::ProtocolConnection;
use thiserror::Error;

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

// Error Type Struct for wrapping errors
#[derive(Error, Debug)]
pub enum VeriflowError {
    // IO Error
    #[error("Network/Disk Error: {0}")]
    Io(#[from] std::io::Error),

    // JSON Error
    #[error("Serialisation Error: {0}")]
    JSON(#[from] serde_json::Error),

    // File Path Error
    #[error("Invalid Path: Could not extract a valid filename from the provided path")]
    InvalidPath,
}

// Allow writing Result<String> instead of Result<String, VeriflowError>
pub type Result<T> = std::result::Result<T, VeriflowError>;

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpStream;
    // Test Serialisation and Deserialisation
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
        let _result = connection.send_header(&json_string).await?;
        let header_length = connection.read_prefix().await?;
        let byte_header = connection.read_body(header_length).await?;
        let header = String::from_utf8_lossy(&byte_header);
        assert_eq!(json_string, header);

        Ok(())
    }
    // Test VeriFlow error type struct
    #[test]
    fn test_error_conversion() {
        // parse non-json into FileHeader
        fn json_fail() -> super::Result<FileHeader> {
            let garbage = "not json";
            // 'from_str' tries to convert string to JSON,
            // the '?' operator handles the failure by automatically converting the JSON error to custom wrapper error
            let header: FileHeader = serde_json::from_str(garbage)?;
            Ok(header)
        }

        let result = json_fail();

        // test if resturned type is an Error
        assert!(result.is_err());

        // Verify the error type
        println!("{}", result.unwrap_err());
    }
}
