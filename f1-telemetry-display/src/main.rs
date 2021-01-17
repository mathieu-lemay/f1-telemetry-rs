#[macro_use]
extern crate log;
extern crate simplelog;

use std::fs::OpenOptions;

use f1_telemetry::Stream;
use simplelog::*;

use crate::ui::get_ui;

mod fmt;
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

    let mut ui = get_ui(true);

    ui.run(stream);

    ui.destroy();
}
