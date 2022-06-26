use std::net::SocketAddr;
use std::time::Duration;

use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use log::{error, info, LevelFilter};
use simplelog::{
    format_description, ColorChoice, CombinedLogger, ConfigBuilder, LevelPadding, TermLogger,
    TerminalMode,
};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::{Error, Message, Result};

use f1_telemetry::Stream;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct AppArgs {
    /// Host to bind on for the UDP packet listener
    #[clap(short, long, default_value = "0.0.0.0")]
    host: String,

    /// port to bind on for the UDP packet listener
    #[clap(short, long, default_value = "20777")]
    port: u16,

    /// Host to bind on for the websocket server
    #[clap(long, default_value = "0.0.0.0")]
    server_host: String,

    /// Port to bind on for the websocket server
    #[clap(long, default_value = "20888")]
    server_port: u16,
}

fn init_logger() {
    let app_logger_config = ConfigBuilder::new()
        .set_level_padding(LevelPadding::Right)
        .set_target_level(LevelFilter::Error)
        .set_location_level(LevelFilter::Debug)
        .set_thread_level(LevelFilter::Off)
        .set_time_format_custom(format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3][offset_hour sign:mandatory]:[offset_minute]"))
        // .set_time_format_custom(format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"))
        // .set_time_offset_to_local().unwrap()
        // .add_filter_allow_str("pipeline_runner")
        .build();

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        app_logger_config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .expect("Error initializing logger");
}

#[tokio::main]
async fn main() {
    init_logger();

    let args = AppArgs::parse();

    let addr = format!("{}:{}", args.host, args.port);
    let packet_stream = Stream::new(&addr).expect("Unable to bind packet socket");
    info!("Listening for telemetry packets on: {}", addr);

    let (tx, _) = broadcast::channel(32);
    let mut interval = tokio::time::interval(Duration::from_millis(5));

    let packet_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            match packet_stream.next() {
                Ok(p) => match p {
                    Some(p) => {
                        let value = serde_json::to_string(&p).unwrap();
                        let _ = packet_tx.send(value);
                    }
                    None => {
                        interval.tick().await;
                    }
                },
                Err(err) => {
                    error!("{:?}", err);
                }
            }
        }
    });

    let addr = format!("{}:{}", args.server_host, args.server_port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Unable to bind server socket");
    info!("Listening for websockets on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        let rx = tx.subscribe();

        tokio::spawn(accept_connection(peer, stream, rx));
    }
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream, rx: Receiver<String>) {
    if let Err(e) = handle_connection(peer, stream, rx).await {
        match e {
            Error::ConnectionClosed | Error::Utf8 => (),
            Error::Protocol(err) => info!("Protocol error: {}: {}", err, peer),
            err => error!("Error processing connection: {}: {}", err, peer),
        }
    }
}

async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
    mut rx: Receiver<String>,
) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, _) = ws_stream.split();

    loop {
        match rx.recv().await {
            Ok(p) => {
                ws_sender.send(Message::Text(p)).await?;
            }
            Err(_e) => {
                error!("{:?}", _e);
            }
        }
    }
}
