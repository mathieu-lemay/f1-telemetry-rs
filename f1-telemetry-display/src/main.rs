#[macro_use]
extern crate log;
extern crate simplelog;

use std::fs::OpenOptions;

use clap::{App, Arg, ArgMatches};
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

fn get_cli_args<'a>() -> ArgMatches<'a> {
    App::new("F1 Telemetry Display")
        .version("1.0")
        .about("Display telemetry info from F1 games")
        .arg(
            Arg::with_name("host")
                .long("host")
                .short("h")
                .default_value("0.0.0.0")
                .help("Host to bind on"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .default_value("20777")
                .help("Port to bind on"),
        )
        .arg(
            Arg::with_name("ui")
                .long("ui")
                .short("u")
                .possible_values(&["ncurses", "gtk"])
                .default_value("gtk")
                .help("Choose the user interfaec"),
        )
        .get_matches()
}

fn main() {
    init_logger();
    let args = get_cli_args();

    let host = args.value_of("host").unwrap();
    let port = args.value_of("port").unwrap();
    let ui = args.value_of("ui").unwrap();

    let stream = Stream::new(format!("{}:{}", host, port)).expect("Unable to bind socket");

    info!("Listening on {}", stream.socket().local_addr().unwrap());

    let mut ui = get_ui(ui);

    ui.run(stream);

    ui.destroy();
}
