use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::config::COMMAND_SERVER;

pub(crate) async fn connect_to_server() -> TcpStream {
    // Use tokio::net::TcpStream to establish an async connection
    match TcpStream::connect(COMMAND_SERVER).await {
        Ok(stream) => stream,
        Err(error) => panic!("Error connecting to server: {}", error),
    }
}