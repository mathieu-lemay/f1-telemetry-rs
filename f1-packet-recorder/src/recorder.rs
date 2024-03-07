use std::net::UdpSocket;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::thread::{sleep, spawn};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use log::{info, warn};
use rusqlite::Connection;
use time::Instant;

use crate::utils::{ctrl_c_channel, get_database_connection};
use f1_telemetry::packet::{parse_packet, Packet};

use super::RecordArgs;

pub(crate) fn record(args: &RecordArgs) -> Result<()> {
    info!("Recording {}:{} to {}", args.host, args.port, args.file);

    let recorder = Recorder::new(args)?;
    let ctrl_receiver = ctrl_c_channel()?;

    recorder.record(&ctrl_receiver)?;

    Ok(())
}

struct TimestampedPacket {
    timestamp: f64,
    packet: Packet,
    raw_packet: Vec<u8>,
}

struct Recorder {
    host: String,
    port: u16,
    conn: Connection,
}

impl Recorder {
    fn new(args: &RecordArgs) -> Result<Self> {
        let conn = get_database_connection(&args.file)?;

        Ok(Self {
            host: args.host.clone(),
            port: args.port,
            conn,
        })
    }

    fn record(&self, ctrl_receiver: &Receiver<()>) -> Result<()> {
        self.init_db()?;

        let rx = self.start_receiver_socket()?;
        let mut last_recv = Instant::now();
        let mut packets: Vec<TimestampedPacket> = Vec::with_capacity(256);

        loop {
            if ctrl_receiver.try_recv().is_ok() {
                info!("Stopping recording");
                break;
            }

            let packet = match rx.try_recv() {
                Ok(p) => Some(p),
                Err(TryRecvError::Empty) => None,
                Err(e) => {
                    warn!("Error receiving packet: {:?}", e);
                    continue;
                }
            };

            if packet.is_none() {
                if !packets.is_empty() && last_recv.elapsed() >= Duration::from_secs(5) {
                    info!("No packets received in the last 5 sec, saving packets to database.");

                    self.save_packets(&mut packets)?;
                }
                sleep(Duration::from_millis(1));
                continue;
            }

            packets.push(packet.unwrap());
            if packets.len() >= 256 {
                info!("Saving packets to database.");
                self.save_packets(&mut packets)?;
            }

            last_recv = Instant::now();
        }

        info!("Saving packets to database.");
        self.save_packets(&mut packets)?;

        Ok(())
    }

    fn init_db(&self) -> Result<()> {
        let create_table_stmt = "
            CREATE TABLE IF NOT EXISTS packets (
                pkt_id           INTEGER  PRIMARY KEY,
                timestamp        REAL     NOT NULL,
                packetFormat     INTEGER  NOT NULL,
                gameMajorVersion INTEGER  NOT NULL,
                gameMinorVersion INTEGER  NOT NULL,
                packetVersion    INTEGER  NOT NULL,
                packetID         INTEGER  NOT NULL,
                sessionID        CHAR(16) NOT NULL,
                sessionTime      REAL     NOT NULL,
                frameIdentifier  INTEGER  NOT NULL,
                playerCarIndex   INTEGER  NOT NULL,
                packet           BLOB     NOT NULL
            );
        ";

        self.conn.execute(create_table_stmt, ())?;

        Ok(())
    }

    fn start_receiver_socket(&self) -> Result<Receiver<TimestampedPacket>> {
        let socket = UdpSocket::bind(format!("{}:{}", self.host, self.port))?;
        let (tx, rx) = channel();

        spawn(move || {
            loop {
                let mut buf = [0; 2048]; // All packets fit in 2048 bytes
                match socket.recv(&mut buf) {
                    Ok(len) => match process_incoming_packet(len, &buf) {
                        Ok(p) => {
                            let _ = tx.send(p);
                        }
                        Err(e) => warn!("Error processing packet: {:?}", e),
                    },
                    Err(e) => {
                        warn!("Invalid packet received: {:?}", e);
                    }
                }
            }
        });

        Ok(rx)
    }

    fn save_packets(&self, packets: &mut Vec<TimestampedPacket>) -> Result<()> {
        let mut stmt = self.conn.prepare_cached("
            INSERT INTO packets(
                timestamp, packetFormat, gameMajorVersion, gameMinorVersion, packetVersion, packetID,
                sessionID, sessionTime, frameIdentifier, playerCarIndex, packet
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
        ")?;

        self.conn.execute("BEGIN;", ())?;
        for p in packets.iter() {
            let header = p.packet.header();
            stmt.execute((
                p.timestamp,
                header.packet_format,
                header.game_major_version,
                header.game_minor_version,
                header.packet_version,
                header.packet_type as u8,
                format!("{:16x}", header.session_uid),
                header.session_time,
                header.frame_identifier,
                header.player_car_index,
                &p.raw_packet,
            ))?;
        }
        self.conn.execute("COMMIT;", ())?;
        packets.clear();

        Ok(())
    }
}

fn process_incoming_packet(len: usize, buf: &[u8]) -> Result<TimestampedPacket> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Somehow we went back in time")
        .as_secs_f64();
    let packet = parse_packet(len, buf)?;

    Ok(TimestampedPacket {
        timestamp,
        packet,
        raw_packet: Vec::from(&buf[..len]),
    })
}
