#[macro_use]
extern crate log;

use clap::{Parser, ValueEnum};
use simplelog::*;

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

#[tokio::main]
async fn main() {
    let args = AppArgs::parse();

    LogBuilder::new()
        .with_file_logger(LevelFilter::Info, "f1-telemetry-display.log")
        .expect("Unable to open log file.")
        .build()
        .expect("Error initializing loggger.");

    let mut ui = get_ui(match args.ui {
        UserInterface::Gtk => "gtk",
        UserInterface::Ncurses => "ncurses",
    });

    ui.run(args.host, args.port).await;

    ui.destroy();
}
