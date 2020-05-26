use packet::participants::PacketParticipantsData;
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

    let mut participants: Option<PacketParticipantsData> = None;

    loop {
        match stream.next() {
            Ok(p) => {
                match p {
                    Some(p) => {
                        match p {
                            Packet::Session(s) => println!(
                                "Session<Type={:?}, Track={:?}, Elapsed={}>",
                                s.session_type(),
                                s.track(),
                                s.session_duration() - s.session_time_left()
                            ),
                            //Packet::Lap(l) => println!("{:?}", l),
                            Packet::Event(e) => {
                                let driver;

                                if let Some(v) = e.vehicle_idx() {
                                    if let Some(p) = &participants {
                                        driver = Some(p.participants()[*v as usize].name());
                                    } else {
                                        driver = None;
                                    }
                                } else {
                                    driver = None;
                                }

                                println!(
                                    "Event<Type={:?}, Vehicle={:?} ({:?}), LapTime={:?}>",
                                    e.event(),
                                    driver,
                                    e.vehicle_idx(),
                                    e.lap_time()
                                );
                            }
                            Packet::Participants(p) => participants = Some(p),
                            Packet::CarTelemetry(t) => {
                                let car_idx = *t.header().player_car_index() as usize;
                                let c = &t.car_telemetry_data()[car_idx];

                                println!(
                                    "Telemetry<Speed={}km/h, RPM={}, Gear={}, Throttle={}%, Brake={}%, Steer={:.02}, Buttons={:?}>",
                                    c.speed(),
                                    c.engine_rpm(),
                                    c.gear(),
                                    (c.throttle() * 100.0) as u32,
                                    (c.brake() * 100.0) as u32,
                                    c.steer(),
                                    t.get_pressed_buttons(),
                                );
                            }
                            //Packet::CarStatus(s) => println!("{:?}", s),
                            _ => {}
                        }
                    }
                    None => sleep(Duration::from_millis(5)),
                }
            }
            Err(_e) => {
                //println!("{:?}", _e);
            }
        }
    }
}
