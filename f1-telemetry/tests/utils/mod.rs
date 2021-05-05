use tokio::net::UdpSocket;

use f1_telemetry::Stream;

pub async fn get_stream() -> Stream {
    Stream::new("127.0.0.1:0")
        .await
        .expect("Unable to bind socket")
}

pub async fn get_connected_socket(stream: &Stream) -> &UdpSocket {
    let s = stream.socket();
    let addr = s.local_addr().expect("Unable to get socket local address");

    s.connect(addr).await.expect("Unable to connect socket");

    s
}

#[allow(dead_code)]
pub async fn send_raw_data(stream: &Stream, data: &str) {
    let data = hex::decode(data).unwrap();
    let socket = get_connected_socket(stream).await;

    let res = socket.send(&data).await;

    assert!(res.is_ok(), "Error sending data to socket");
    assert_eq!(
        res.unwrap(),
        data.len(),
        "Invalid amount of data sent through socket"
    );
}
