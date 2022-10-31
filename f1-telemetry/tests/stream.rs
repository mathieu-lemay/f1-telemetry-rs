use f1_telemetry::packet::UnpackError;

mod utils;

#[tokio::test]
async fn test_invalid_packet_returns_an_error() {
    let stream = utils::get_stream().await;
    let socket = utils::get_connected_socket(&stream).await;

    let data = vec![0xe4, 0x07];
    let res = socket.send(&data).await;

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), data.len());

    let p = stream.next().await;

    assert_eq!(
        p.unwrap_err(),
        UnpackError(String::from(
            "Packet too small: 2 bytes (minimum: 24 bytes)"
        ))
    );
}
