#[macro_use]
extern crate log;

use std::net::SocketAddr;

use tokio::net::{ToSocketAddrs, UdpSocket};

use packet::{parse_packet, Packet, UnpackError};

mod f1_2019;
mod f1_2020;
pub mod packet;
mod utils;

pub struct Stream {
    socket: UdpSocket,
}

impl Stream {
    pub async fn new<T: ToSocketAddrs>(addr: T) -> std::io::Result<Stream> {
        let socket = UdpSocket::bind(addr).await?;

        info!("Listening on {}", socket.local_addr().unwrap());

        Ok(Stream { socket })
    }

    pub async fn next(&self) -> Result<Packet, UnpackError> {
        let mut buf = [0; 2048]; // All packets fit in 2048 bytes

        match self.socket.recv(&mut buf).await {
            Ok(len) => parse_packet(len, &buf),
            Err(e) => Err(UnpackError(format!("Error reading from socket: {:?}", e))),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.socket.local_addr().unwrap()
    }
}
