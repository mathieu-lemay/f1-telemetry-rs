use std::net::UdpSocket;
use std::sync::mpsc::Receiver;
use std::thread::sleep;
use std::time::Duration;

use anyhow::{Error, Result};
use log::info;
use rusqlite::Connection;
use time::Instant;

use crate::utils::{ctrl_c_channel, get_database_connection};

use super::PlayArgs;

pub(crate) fn play(args: &PlayArgs) -> Result<()> {
    info!(
        "Replaying {} to {}:{} (realtime: {}/{:.1}x, loop: {})",
        args.file,
        args.destination
            .as_ref()
            .unwrap_or(&String::from("<broadcast>")),
        args.port,
        args.realtime,
        args.realtime_factor,
        args.loop_play
    );

    let player = Player::new(args)?;
    let ctrl_receiver = ctrl_c_channel()?;

    loop {
        player.play(&ctrl_receiver)?;

        if !args.loop_play {
            break;
        }
    }

    Ok(())
}

fn get_socket(destination: &Option<String>, port: u16) -> Result<UdpSocket> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    match destination {
        Some(d) => socket.connect(format!("{}:{}", d, port))?,
        None => {
            socket.connect(format!("0.0.0.0:{}", port))?;
            socket.set_broadcast(true)?;
        }
    };

    Ok(socket)
}

struct TimestampedPacket {
    timestamp: f64,
    data: Vec<u8>,
}

struct Player {
    socket: UdpSocket,
    conn: Connection,

    realtime: bool,
    realtime_factor: f32,
    skip: u64,
}

impl Player {
    fn new(args: &PlayArgs) -> Result<Self> {
        let socket = get_socket(&args.destination, args.port)?;
        let conn = get_database_connection(&args.file)?;

        Ok(Self {
            socket,
            conn,
            realtime: args.realtime,
            realtime_factor: args.realtime_factor,
            skip: args.skip,
        })
    }

    fn play(&self, ctrl_receiver: &Receiver<()>) -> Result<()> {
        let playback_start = Instant::now();
        let first_timestamp = self.get_first_timestamp()?;

        let mut stmt = self
            .conn
            .prepare("SELECT timestamp, packet FROM packets ORDER BY pkt_id LIMIT -1 OFFSET ?;")?;

        let packets = stmt.query_map([self.skip], |r| {
            Ok(TimestampedPacket {
                timestamp: r.get(0)?,
                data: r.get(1)?,
            })
        })?;

        for (idx, packet) in packets.enumerate() {
            if ctrl_receiver.try_recv().is_ok() {
                info!("Stopping playback");
                return Err(Error::msg("ctrl-c received"));
            }

            let packet = packet?;

            self.delay_next_packet(playback_start, first_timestamp, packet.timestamp);

            self.socket.send(&packet.data)?;

            if (idx + 1) % 500 == 0 {
                let since_start = playback_start.elapsed().as_seconds_f64();
                let expected_elapsed = packet.timestamp - first_timestamp;

                info!(
                    "{} packages sent, delay: {:.3}ms",
                    idx + 1,
                    (since_start - expected_elapsed) * 1000.0
                )
            }
        }

        Ok(())
    }

    fn get_first_timestamp(&self) -> Result<f64> {
        let mut stmt = self.conn.prepare(
            "SELECT timestamp FROM packets WHERE pkt_id = (SELECT min(pkt_id) FROM packets);",
        )?;

        let mut rows = stmt.query([])?;

        Ok(rows.next()?.unwrap().get(0)?)
    }

    fn delay_next_packet(
        &self,
        playback_start: Instant,
        first_packet_timestamp: f64,
        packet_timestamp: f64,
    ) {
        let duration = if self.realtime {
            let expected_delay =
                (packet_timestamp - first_packet_timestamp) / self.realtime_factor as f64;
            let real_delay = playback_start.elapsed().as_seconds_f64();

            if real_delay < expected_delay {
                let delta = expected_delay - real_delay;
                Some(Duration::from_secs_f64(delta))
            } else {
                None
            }
        } else {
            Some(Duration::from_millis(1))
        };

        if let Some(d) = duration {
            sleep(d);
        }
    }
}
