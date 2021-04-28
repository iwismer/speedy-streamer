use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

/// Holds a connection to a single client, and forwards reads to it.
#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
    addr: SocketAddr,
}

impl Client {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Result<Client, &'static str> {
        Ok(Client {
            stream: stream,
            addr,
        })
    }

    /// Send a single read to the connected client.
    pub async fn send_read(&mut self, read: String) -> Result<usize, SocketAddr> {
        self.stream
            .write(read.as_bytes())
            .await
            .map_err(|_| self.addr)
    }

    /// Close the connection to the client.
    pub async fn exit(&mut self) {
        match self.stream.shutdown().await {
            Ok(_) => println!("\r\x1b[2KClient disconnected gracefully."),
            Err(e) => eprintln!("\r\x1b[2KError disconnecting: {}", e),
        };
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }
}
