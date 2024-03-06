use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use log::LevelFilter;
use simplelog::{ColorChoice, TerminalMode};

use f1_telemetry_common::logging::LogBuilder;

mod player;
mod recorder;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Replay a recorded session
    Play(PlayArgs),

    /// Record an incoming session
    Record(RecordArgs),
}

#[derive(Debug, Args)]
struct PlayArgs {
    /// Database file to replay
    // #[clap(long, default_value = None)]
    file: String,

    /// Destination IP address. Leave empty to use broadcast.
    #[clap(long, default_value = None)]
    destination: Option<String>,

    /// Port to write to.
    #[clap(long, default_value = "20777")]
    port: u16,

    /// Loop over the database
    #[clap(long = "loop")]
    loop_play: bool,

    /// Play the packets in realtime or not.
    #[clap(long)]
    realtime: bool,

    /// Real-time playback factor (higher is faster)
    #[clap(long, default_value = "1.0")]
    realtime_factor: f32,

    /// Number of packets to skip at the start of the file
    #[clap(long, default_value = "0")]
    skip: u64,
}

#[derive(Debug, Args)]
struct RecordArgs {
    /// Database file to record to
    #[clap(short, long, default_value = None)]
    file: String,

    /// Host to bind on for the UDP packet listener
    #[clap(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to bind on for the UDP packet listener
    #[clap(long, default_value = "20777")]
    port: u16,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    LogBuilder::new()
        .with_term_logger(LevelFilter::Info, TerminalMode::Mixed, ColorChoice::Auto)
        .build()
        .expect("Error initializing logger.");

    match &cli.command {
        Commands::Play(args) => player::play(args),
        Commands::Record(args) => recorder::record(args),
    }
}
