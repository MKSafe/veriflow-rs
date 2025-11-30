mod client{
    use tokio::net::TcpStream;
    use std::io;
    struct Sender{
        buffer: [u8;500],
        stream: Option<TcpStream>,
        host: String,
        port: String,
    }

    impl Sender{
        pub async fn new(host: &str,port: &str,) -> io::Result<Sender>{
            if host == "" || port ==""{
                return Ok(Sender {buffer:[0;500], stream: None, host: "".to_string(), port: "".to_string(),})
            }
            let addr = format("{}:{}",host,port);
            let stream = TcpStream.connect(addr).await?;
            return Ok(Sender {buffer: [0;500], stream: Some(stream), host: host.to_string(),port: port.to_string()})
        }
    }
}