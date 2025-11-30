mod Server{
    use tokio::net::TcpListener
    pub struct Listener{
        host: String
        port: String
        listener: Option<TcpListener>
        header_buffer:[u8; 4]
        buffer: [u8; 500]
        open_connections: Vec<T>
    }

    impl Listener{
        fn new(host:&str,port:&str,){
            
        }
    }
}