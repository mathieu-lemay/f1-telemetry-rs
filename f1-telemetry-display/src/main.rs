use f1_telemetry::packet::lap::PacketLapData;
use f1_telemetry::packet::participants::PacketParticipantsData;
use f1_telemetry::packet::Packet;
use f1_telemetry::Stream;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let stream = Stream::new("0.0.0.0:20777").expect("Unable to bind socket");
    println!("Listening on {}", stream.socket().local_addr().unwrap());

    let mut participants: Option<PacketParticipantsData> = None;

    ncurses::setlocale(ncurses::LcCategory::all, "");

    let hwnd = ncurses::initscr();

    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::keypad(hwnd, true);
    ncurses::timeout(0);

    ncurses::refresh();

    loop {
        match stream.next() {
            Ok(p) => match p {
                Some(p) => match p {
                    Packet::Lap(ld) => {
                        if let Some(p) = &participants {
                            render_lap_data(&ld, p)
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
                _ => {}
            }
        }
    }

    ncurses::endwin();
}

fn render_lap_data(lap_data: &PacketLapData, participants: &PacketParticipantsData) {
    for (i, ld) in lap_data.lap_data().iter().enumerate() {
        let pos = ld.car_position();
        let name = participants.participants()[i].name();
        let s = format!(
            "{:2}. {:20} | {} | {} | {}",
            pos,
            name,
            format_time(ld.current_lap_time()),
            format_time(ld.last_lap_time()),
            format_time(ld.best_lap_time())
        );

        ncurses::mvaddstr(pos as i32 - 1, 0, s.as_str());
        ncurses::clrtoeol();
    }

    ncurses::refresh();
}

fn format_time(ts: f32) -> String {
    let seconds = ts as i64;
    let millis = ((ts - ts.floor()) * 1000.0).round();

    let hours = seconds / 3600;
    let minutes = (seconds - hours * 3600) / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}
