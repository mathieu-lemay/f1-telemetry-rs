#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use clap::{Parser, ValueEnum};
use simplelog::*;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::RwLock;

use f1_telemetry::packet::Packet;
use f1_telemetry::Stream;
use f1_telemetry_common::logging::LogBuilder;

use crate::ui::get_ui;

mod fmt;
mod models;
mod ui;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, ValueEnum)]
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
    let args = AppArgs::parse();

    let mut log_builder = LogBuilder::new()
        .with_file_logger(LevelFilter::Info, "f1-telemetry-display.log")
        .expect("Unable to open log file.");

    if args.ui == UserInterface::Gtk {
        log_builder =
            log_builder.with_term_logger(LevelFilter::Info, TerminalMode::Mixed, ColorChoice::Auto);
    }

    log_builder.build().expect("Error initializing logger.");

    start_stream(args.host, args.port).await;
    run(&args.ui).await;
}

async fn start_stream(host: String, port: u16) {
    let stream = Stream::new(format!("{}:{}", host, port))
        .await
        .expect("Unable to bind socket");

    info!("Listening on {}", stream.socket().local_addr().unwrap());

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

async fn run(ui_type: &UserInterface) {
    let mut ui = get_ui(match ui_type {
        UserInterface::Gtk => "gtk",
        UserInterface::Ncurses => "ncurses",
    });
    ui.run().await;
    ui.destroy();
}
