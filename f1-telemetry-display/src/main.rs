#[macro_use]
extern crate log;
extern crate simplelog;

use std::fs::OpenOptions;

use clap::{Parser, ValueEnum};
use simplelog::*;

use f1_telemetry::Stream;

use crate::ui::get_ui;

mod fmt;
mod models;
mod ui;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum UserInterface {
    Gtk,
    Ncurses,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
struct AppArgs {
    /// Host to bind on for the UDP packet listener
    #[clap(long, default_value = "0.0.0.0")]
    host: String,

    /// port to bind on for the UDP packet listener
    #[clap(long, default_value = "20777")]
    port: u16,

    #[arg(long, value_enum, default_value = "gtk")]
    ui: UserInterface,
}

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
    let args = AppArgs::parse();

    let stream =
        Stream::new(format!("{}:{}", args.host, args.port)).expect("Unable to bind socket");

    info!("Listening on {}", stream.socket().local_addr().unwrap());

    let mut ui = get_ui(match args.ui {
        UserInterface::Gtk => "gtk",
        UserInterface::Ncurses => "ncurses",
    });

    ui.run(stream);

    ui.destroy();
}
