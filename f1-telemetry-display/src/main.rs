use f1_telemetry::packet::lap::{PacketLapData, PitStatus};
use f1_telemetry::packet::participants::PacketParticipantsData;
use f1_telemetry::packet::session::PacketSessionData;
use f1_telemetry::packet::Packet;
use f1_telemetry::Stream;
use models::{LapInfo, SessionInfo};
use std::thread::sleep;
use std::time::Duration;
use ui::Ui;

mod models;
mod ui;

fn main() {
    let stream = Stream::new("0.0.0.0:20777").expect("Unable to bind socket");
    println!("Listening on {}", stream.socket().local_addr().unwrap());

    let mut participants: Option<PacketParticipantsData> = None;
    let mut current_lap: u8 = 0;

    let ui = Ui::init();

    loop {
        match stream.next() {
            Ok(p) => match p {
                Some(p) => match p {
                    Packet::Session(s) => {
                        let sinfo = parse_session_data(&s, current_lap);
                        ui.print_session_info(&sinfo);
                    }
                    Packet::Lap(ld) => {
                        current_lap = get_current_lap(&ld);
                        if let Some(lap_info) = parse_lap_data(&ld, &participants) {
                            ui.print_lap_info(&lap_info);
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

fn parse_session_data(session: &PacketSessionData, current_lap: u8) -> SessionInfo {
    SessionInfo {
        session_name: session.session_type().name(),
        track_name: session.track().name(),
        elapsed_time: session.session_duration() - session.session_time_left(),
        duration: session.session_duration(),
        current_lap,
        number_of_laps: session.total_laps(),
    }
}

fn parse_lap_data<'a>(
    lap_data: &'a PacketLapData,
    participants: &'a Option<PacketParticipantsData>,
) -> Option<Vec<LapInfo<'a>>> {
    if participants.is_none() {
        return None;
    }

    let participants = participants.as_ref().unwrap().participants();

    let mut lap_info = Vec::with_capacity(lap_data.lap_data().len());

    for (i, ld) in lap_data.lap_data().iter().enumerate() {
        let name = participants[i].name();
        let team = participants[i].team();

        let li = LapInfo {
            position: ld.car_position(),
            name,
            team,
            current_lap_time: ld.current_lap_time(),
            last_lap_time: ld.last_lap_time(),
            best_lap_time: ld.best_lap_time(),
            status: ld.result_status(),
            in_pit: ld.pit_status() != PitStatus::None,
            lap_invalid: ld.current_lap_invalid(),
        };

        lap_info.push(li);
    }

    Some(lap_info)
}

fn get_current_lap(lap_data: &PacketLapData) -> u8 {
    lap_data
        .lap_data()
        .iter()
        .map(|l| l.current_lap_num())
        .max()
        .unwrap_or(0)
}
