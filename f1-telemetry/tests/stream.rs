use serial_test::serial;

use f1_telemetry::packet::UnpackError;

mod utils;

#[test]
#[serial]
fn test_empty_stream_returns_nothing() {
    let stream = utils::get_stream();

    let p = stream.next();

    assert!(p.unwrap().is_none());
}

#[test]
#[serial]
fn test_invalid_packet_returns_an_error() {
    let stream = utils::get_stream();
    let socket = utils::get_connected_socket(&stream);

    let data = vec![0xe4, 0x07];
    let res = socket.send(&data);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), data.len());

    let p = utils::get_packet(&stream);

    assert_eq!(
        p.unwrap_err(),
        UnpackError(String::from(
            "Packet too small: 2 bytes (minimum: 24 bytes)"
        ))
    );
}
