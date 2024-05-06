use std::net::SocketAddr;

use anyhow::Result;
use tokio::{io::AsyncWriteExt as _, net::TcpListener};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";

    let listener = TcpListener::bind(addr).await?;
    info!("Dredis: listening on: {}", addr);

    loop {
        let (socket, raddr) = listener.accept().await?;
        info!("Dredis: connection from: {}", raddr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(socket, raddr).await {
                warn!("Dredis: Error processing conn with {}: {:?}", raddr, e);
            }
        });
    }
}

async fn process_redis_conn(mut socket: tokio::net::TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        socket.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        match socket.try_read_buf(&mut buf) {
            Ok(0) => {
                info!("Dredis: read EOF");
                break;
            }
            Ok(n) => {
                info!("Dredis: read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("Dredis: read: {:?}", line);
                socket.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                info!("Dredis: read would block");
                continue;
            }
            Err(e) => {
                info!("Dredis: read error: {:?}", e);
                break;
            }
        }
    }
    warn!("Dredis: connection closed: {}", raddr);
    Ok(())
}
