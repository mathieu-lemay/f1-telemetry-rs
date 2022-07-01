use std::net::UdpSocket;
use std::{thread, time};

use f1_telemetry::packet::{Packet, UnpackError};
use f1_telemetry::Stream;

pub fn get_stream() -> Stream {
    Stream::new("127.0.0.1:0").expect("Unable to bind socket")
}

pub fn get_connected_socket(stream: &Stream) -> &UdpSocket {
    let s = stream.socket();
    let addr = s.local_addr().expect("Unable to get socket local address");

    s.connect(addr).expect("Unable to connect socket");

    s
}

#[allow(dead_code)]
pub fn send_raw_data(stream: &Stream, data: &str) {
    let data = hex::decode(data).unwrap();
    let socket = get_connected_socket(stream);

    let res = socket.send(&data);

    assert!(res.is_ok(), "Error sending data to socket");
    assert_eq!(
        res.unwrap(),
        data.len(),
        "Invalid amount of data sent through socket"
    );
}

pub fn get_packet(stream: &Stream) -> Result<Option<Packet>, UnpackError> {
    let sleep_duration = time::Duration::from_millis(50);

    for _ in 0..10 {
        let res = stream.next();

        match res {
            Ok(Some(_)) | Err(_) => {
                return res;
            }
            _ => {}
        }

        thread::sleep(sleep_duration);
    }

    return Ok(None);
}
