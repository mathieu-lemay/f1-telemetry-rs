use std::net::UdpSocket;

use f1_telemetry::Stream;

static UDP_BIND_ADDRESS: &str = "127.0.0.1:20777";

pub fn get_stream() -> Stream {
    Stream::new(UDP_BIND_ADDRESS).expect("Unable to bind socket")
}

pub fn get_connected_socket(stream: &Stream) -> &UdpSocket {
    let s = stream.socket();
    s.connect(UDP_BIND_ADDRESS)
        .expect("Unable to connect socket");

    s
}

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
