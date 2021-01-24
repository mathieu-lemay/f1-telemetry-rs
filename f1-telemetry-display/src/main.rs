#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate simplelog;

use std::fs::OpenOptions;

use clap::{App, Arg};
use simplelog::*;

use crate::ui::get_ui;
use f1_telemetry::packet::Packet;
use f1_telemetry::Stream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::RwLock;

mod fmt;
mod models;
mod ui;

fn init_logger(log_level: LevelFilter) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("f1-telemetry-display.log")
        .expect("Unable to open log file.");
    let config = ConfigBuilder::new().set_time_to_local(true).build();

    WriteLogger::init(log_level, config, file).expect("Unable to initialize logger.");
}

struct AppArgs {
    host: String,
    port: u16,
    ui: String,
    log_level: LevelFilter,
}

fn is_valid_port(val: String) -> Result<(), String> {
    match val.parse::<u16>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Invalid port: {}", val)),
    }
}

fn is_valid_log_level(val: String) -> Result<(), String> {
    match val.parse::<LevelFilter>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Invalid log level: {}", val)),
    }
}

fn get_cli_args() -> AppArgs {
    let args = App::new("F1 Telemetry Display")
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
                .validator(is_valid_port)
                .help("Port to bind on"),
        )
        .arg(
            Arg::with_name("ui")
                .long("ui")
                .short("u")
                .possible_values(&["ncurses", "gtk"])
                .default_value("gtk")
                .help("Choose the user interface"),
        )
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .short("l")
                .default_value("info")
                .validator(is_valid_log_level)
                .help("Set the log level"),
        )
        .get_matches();

    let host = args.value_of("host").unwrap().to_string();
    let port = args
        .value_of("port")
        .unwrap()
        .parse::<u16>()
        .expect("Invalid port");
    let ui = args.value_of("ui").unwrap().to_string();
    let log_level = args
        .value_of("log_level")
        .unwrap()
        .parse::<LevelFilter>()
        .unwrap();

    AppArgs {
        host,
        port,
        ui,
        log_level,
    }
}

struct StaticChannel {
    tx: UnboundedSender<Packet>,
    rx: RwLock<UnboundedReceiver<Packet>>,
}

lazy_static! {
    pub(crate) static ref CHANNEL: StaticChannel = {
        let (tx, rx) = mpsc::unbounded_channel();

        StaticChannel {
            tx,
            rx: RwLock::new(rx),
        }
    };
}

#[tokio::main]
async fn main() {
    let args = get_cli_args();

    init_logger(args.log_level);
    start_stream(args.host, args.port).await;
    run(&args.ui).await;
}

async fn start_stream(host: String, port: u16) {
    let stream = Stream::new(format!("{}:{}", host, port))
        .await
        .expect("Unable to bind socket");

    tokio::spawn(async move {
        loop {
            match stream.next().await {
                Ok(p) => {
                    let _ = CHANNEL.tx.send(p);
                }
                Err(_e) => {
                    error!("{:?}", _e);
                }
            }
        }
    });
}

async fn run(ui_type: &str) {
    let mut ui = get_ui(ui_type);
    ui.run().await;
    ui.destroy();
}
