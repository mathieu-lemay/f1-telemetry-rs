use std::net::SocketAddr;

use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::runtime::Runtime;

use packet::{parse_packet, Packet, UnpackError};

mod f1_2019;
mod f1_2020;
mod f1_2021;
mod f1_2022;
mod f1_2023;
pub mod packet;
mod utils;

pub struct Stream {
    socket: UdpSocket,
}

impl Stream {
    pub async fn new<T: ToSocketAddrs>(addr: T) -> std::io::Result<Stream> {
        let socket = UdpSocket::bind(addr).await?;

        Ok(Stream { socket })
    }

    pub async fn next(&self) -> Result<Packet, UnpackError> {
        let mut buf = [0; 2048]; // All packets fit in 2048 bytes

        match self.socket.recv(&mut buf).await {
            Ok(len) => parse_packet(len, &buf),
            Err(e) => Err(UnpackError(format!("Error reading from socket: {:?}", e))),
        }
    }

    pub async fn next_from(&self) -> Result<(Packet, SocketAddr), UnpackError> {
        let mut buf = [0; 2048]; // All packets fit in 2048 bytes

        match self.socket.recv_from(&mut buf).await {
            Ok((len, addr)) => parse_packet(len, &buf).map(|p| (p, addr)),
            Err(e) => Err(UnpackError(format!("Error reading from socket: {:?}", e))),
        }
    }

    pub fn socket(&self) -> &UdpSocket {
        &self.socket
    }
}

pub struct SyncStream {
    stream: Stream,
    rt: Runtime,
}

impl SyncStream {
    pub fn new<T: ToSocketAddrs>(addr: T) -> std::io::Result<Self> {
        let rt = Runtime::new().unwrap();
        let stream = rt.block_on(Stream::new(addr))?;

        Ok(SyncStream { stream, rt })
    }

    pub fn next(&self) -> Result<Packet, UnpackError> {
        self.rt.block_on(self.stream.next())
    }

    pub fn next_from(&self) -> Result<(Packet, SocketAddr), UnpackError> {
        self.rt.block_on(self.stream.next_from())
    }
}
