use f1_telemetry::Stream;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let stream = Stream::new("0.0.0.0:20777").expect("Unable to bind socket");
    println!("Listening on {}", stream.socket().local_addr().unwrap());

    loop {
        match stream.next() {
            Ok(p) => match p {
                Some(p) => {
                    println!("{:?}", p);
                }
                None => sleep(Duration::from_millis(5)),
            },
            Err(_e) => {
                panic!("{:?}", _e);
            }
        }
    }
}
