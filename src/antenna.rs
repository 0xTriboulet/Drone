use tokio::net::TcpStream;

pub(crate) async fn connect_to_server(cmd_server:String) -> TcpStream {
    // Use tokio::net::TcpStream to establish an async connection
    match TcpStream::connect(cmd_server).await {
        Ok(stream) => stream,
        Err(error) => panic!("Error connecting to server: {}", error),
    }
}