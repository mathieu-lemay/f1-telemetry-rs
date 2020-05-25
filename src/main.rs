use packet::{parse_packet, Packet, UnpackError};
use std::io::ErrorKind;
use std::net::{ToSocketAddrs, UdpSocket};
use std::thread::sleep;
use std::time::Duration;

mod packet;

struct Stream {
    socket: UdpSocket,
}

impl Stream {
    fn new<T: ToSocketAddrs>(addr: T) -> std::io::Result<Stream> {
        let socket = UdpSocket::bind(addr)?;
        socket.set_nonblocking(true)?;

        Ok(Stream { socket })
    }

    fn next(&self) -> Result<Option<Packet>, UnpackError> {
        let mut buf = [0; 2048]; // All packets fit in 2048 bytes

        match self.socket.recv(&mut buf) {
            Ok(len) => match parse_packet(len, &buf) {
                Ok(p) => Ok(Some(p)),
                Err(e) => Err(e),
            },
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    Ok(None)
                } else {
                    Err(UnpackError(format!("Error reading from socket: {:?}", e)))
                }
            }
        }
    }
}

fn main() {
    let stream = Stream::new("0.0.0.0:20777").expect("Unable to bind socket");
    println!("Listening on {}", stream.socket.local_addr().unwrap());

    let mut c = 0;

    while c < 20 {
        match stream.next() {
            Ok(p) => match p {
                Some(p) => {
                    c += 1;
                    match p {
                        Packet::Session(s) => println!("{:?}", s),
                        Packet::LapData(l) => println!("{:?}", l),
                        Packet::ParticipantsData(p) => println!("{:?}", p),
                    };
                }
                None => sleep(Duration::from_millis(5)),
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
