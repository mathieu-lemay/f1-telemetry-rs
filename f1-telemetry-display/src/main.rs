#[macro_use]
extern crate log;
extern crate simplelog;

use std::fs::OpenOptions;
use std::thread::sleep;
use std::time::Duration;

use f1_telemetry::Stream;
use simplelog::*;

use crate::models::GameState;
use crate::ui::{Ui, View};

mod models;
mod ui;

fn init_logger() {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("f1-telemetry-display.log")
        .expect("Unable to open log file.");
    let config = ConfigBuilder::new().set_time_to_local(true).build();

    WriteLogger::init(LevelFilter::Debug, config, file).expect("Unable to initialize logger.");
}

fn main() {
    init_logger();

    let stream = Stream::new("0.0.0.0:20777").expect("Unable to bind socket");

    info!("Listening on {}", stream.socket().local_addr().unwrap());

    let mut ui = Ui::init();
    let mut game_state = GameState::default();

    loop {
        match stream.next() {
            Ok(p) => match p {
                Some(p) => {
                    game_state.parse_packet(&p);
                    ui.render(&game_state, &p);
                }
                None => sleep(Duration::from_millis(5)),
            },
            Err(_e) => {
                error!("{:?}", _e);
            }
        }

        let ch = ncurses::get_wch();
        if let Some(ch) = ch {
            match ch {
                ncurses::WchResult::Char(49) => {
                    // 1
                    ui.switch_view(View::Dashboard);
                }
                ncurses::WchResult::Char(50) => {
                    // 2
                    ui.switch_view(View::TrackOverview);
                }
                ncurses::WchResult::Char(113) => {
                    // q
                    break;
                }
                // ncurses::WchResult::Char(c) => {
                //     ncurses::mvaddstr(0, 0, format!("Pressed Char: {}", c).as_str());
                // }
                // ncurses::WchResult::KeyCode(c) => {
                //     ncurses::mvaddstr(0, 0, format!("Pressed Key: {}", c).as_str());
                //     ncurses::clrtoeol();
                // }
                _ => {}
            }
        }
    }

    ui.destroy();
}
