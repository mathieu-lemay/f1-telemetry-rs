use f1_telemetry::packet::participants::PacketParticipantsData;
use f1_telemetry::packet::Packet;
use f1_telemetry::Stream;
use std::thread::sleep;
use std::time::Duration;
use ui::Ui;

mod ui;

fn main() {
    let stream = Stream::new("0.0.0.0:20777").expect("Unable to bind socket");
    println!("Listening on {}", stream.socket().local_addr().unwrap());

    let mut participants: Option<PacketParticipantsData> = None;

    let ui = Ui::init();

    loop {
        match stream.next() {
            Ok(p) => match p {
                Some(p) => match p {
                    Packet::Session(s) => ui.print_session_info(&s),
                    Packet::Lap(ld) => {
                        if let Some(p) = &participants {
                            ui.print_lap_data(&ld, p)
                        }
                    }
                    Packet::Participants(p) => participants = Some(p),
                    _ => {}
                },
                None => sleep(Duration::from_millis(5)),
            },
            Err(_e) => {
                panic!("{:?}", _e);
            }
        }

        let ch = ncurses::get_wch();
        if let Some(ch) = ch {
            match ch {
                ncurses::WchResult::Char(113) => {
                    break;
                }
                _ => {} // ncurses::WchResult::Char(c) => {
                        //     ncurses::mvaddstr(23, 0, format!("Pressed Char: {}", c).as_str());
                        // },
                        // ncurses::WchResult::KeyCode(c) => {
                        //     ncurses::mvaddstr(23, 0, format!("Pressed Key: {}", c).as_str());
                        //     ncurses::clrtoeol();
                        // }
            }
        }
    }

    ui.destroy();
}
