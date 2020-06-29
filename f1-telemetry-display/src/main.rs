use std::thread::sleep;
use std::time::Duration;

use f1_telemetry::Stream;

use crate::render::{Renderer, View};

mod models;
mod render;
mod ui;

fn main() {
    let stream = Stream::new("0.0.0.0:20777").expect("Unable to bind socket");
    println!("Listening on {}", stream.socket().local_addr().unwrap());

    let mut renderer = Renderer::new();

    loop {
        match stream.next() {
            Ok(p) => match p {
                Some(p) => {
                    renderer.render(&p);
                }
                None => sleep(Duration::from_millis(5)),
            },
            Err(_e) => {
                panic!("{:?}", _e);
            }
        }

        let ch = ncurses::get_wch();
        if let Some(ch) = ch {
            match ch {
                ncurses::WchResult::Char(49) => {
                    // 1
                    renderer.switch_view(View::Dashboard);
                }
                ncurses::WchResult::Char(50) => {
                    // 2
                    renderer.switch_view(View::Positions);
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

    renderer.destroy();
}
