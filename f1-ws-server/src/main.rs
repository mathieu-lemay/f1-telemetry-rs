use std::io::ErrorKind::BrokenPipe;
use std::net::SocketAddr;

use clap::Parser;
use futures_util::SinkExt;
use log::{error, info, warn, LevelFilter};
use simplelog::{ColorChoice, TerminalMode};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::{Error, Message, Result};

use f1_telemetry::Stream;
use f1_telemetry_common::logging::LogBuilder;

#[derive(Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
struct AppArgs {
    /// Host to bind on for the UDP packet listener
    #[clap(long, default_value = "0.0.0.0", env)]
    listener_host: String,

    /// port to bind on for the UDP packet listener
    #[clap(long, default_value = "20777", env)]
    listener_port: u16,

    /// Host to bind on for the websocket server
    #[clap(long, default_value = "0.0.0.0", env)]
    server_host: String,

    /// Port to bind on for the websocket server
    #[clap(long, default_value = "20888", env)]
    server_port: u16,
}

#[tokio::main]
async fn main() {
    let args = AppArgs::parse();

    LogBuilder::new()
        .with_term_logger(LevelFilter::Info, TerminalMode::Mixed, ColorChoice::Auto)
        .build()
        .expect("Error initializing logger.");

    let addr = format!("{}:{}", args.listener_host, args.listener_port);
    let packet_stream = Stream::new(&addr)
        .await
        .expect("Unable to bind packet socket");
    info!("Listening for telemetry packets on: {}", addr);

    let (tx, _) = broadcast::channel(32);

    let packet_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            match packet_stream.next().await {
                Ok(p) => {
                    let value = serde_json::to_string(&p).unwrap();
                    let _ = packet_tx.send(value);
                }
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
            Error::Io(err) => {
                if err.kind() == BrokenPipe {
                    warn!("Client disconnected: {}: {}", err, peer)
                } else {
                    error!("Error processing connection: {}: {}", err, peer)
                }
            }
            err => error!("Error processing connection: {}: {}", err, peer),
        }
    } else {
        info!("Connection closed: {}", peer);
    }
}

async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
    mut rx: Receiver<String>,
) -> Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);

    loop {
        match rx.recv().await {
            Ok(p) => {
                ws_stream.send(Message::Text(p)).await?;
            }
            Err(_e) => {
                error!("{:?}", _e);
            }
        }
    }
}
