use std::io::Cursor;

use event::parse_event_data;
use header::parse_header;
use lap::parse_lap_data;
use motion::parse_motion_data;
use participants::parse_participants_data;
use session::parse_session_data;

use crate::packet::{Packet, PacketType, UnpackError};

mod consts;
mod event;
mod generic;
mod header;
mod lap;
mod motion;
mod participants;
mod session;

pub(crate) fn parse_packet(size: usize, packet: &[u8]) -> Result<Packet, UnpackError> {
    let mut cursor = Cursor::new(packet);
    let header = parse_header(&mut cursor, size)?;

    match header.packet_type {
        PacketType::Motion => {
            let packet = parse_motion_data(&mut cursor, header, size)?;

            Ok(Packet::Motion(packet))
        }
        PacketType::Session => {
            let packet = parse_session_data(&mut cursor, header, size)?;

            Ok(Packet::Session(packet))
        }
        PacketType::LapData => {
            let packet = parse_lap_data(&mut cursor, header, size)?;

            Ok(Packet::Lap(packet))
        }
        PacketType::Event => {
            let packet = parse_event_data(&mut cursor, header, size)?;

            Ok(Packet::Event(packet))
        }
        PacketType::Participants => {
            let packet = parse_participants_data(&mut cursor, header, size)?;

            Ok(Packet::Participants(packet))
        }
        _ => Err(UnpackError(format!(
            "Not implemented: {:?}",
            header.packet_type
        ))),
    }
}
