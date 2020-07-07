use std::io::Cursor;

use crate::f1_2020::header::{parse_header, validate_header_size};
use crate::f1_2020::lap::parse_lap_data;
use crate::f1_2020::participants::parse_participants_data;
use crate::f1_2020::session::parse_session_data;
use crate::packet::{Packet, PacketType, UnpackError};

mod header;
mod lap;
mod participants;
mod session;

pub(crate) fn parse_packet(size: usize, packet: &[u8]) -> Result<Packet, UnpackError> {
    validate_header_size(size)?;

    let mut cursor = Cursor::new(packet);
    let header = parse_header(&mut cursor);

    let packet_id: PacketType = parse_packet_type(header.packet_id())?;

    match packet_id {
        PacketType::Session => {
            let packet = parse_session_data(&mut cursor, header, size)?;

            Ok(Packet::Session(packet))
        }
        PacketType::LapData => {
            let packet = parse_lap_data(&mut cursor, header, size)?;

            Ok(Packet::Lap(packet))
        }
        PacketType::Participants => {
            let packet = parse_participants_data(&mut cursor, header, size)?;

            Ok(Packet::Participants(packet))
        }
        _ => Err(UnpackError("Not Implemented".to_string())),
    }
}

fn parse_packet_type(value: u8) -> Result<PacketType, UnpackError> {
    match value {
        0 => Ok(PacketType::Motion),
        1 => Ok(PacketType::Session),
        2 => Ok(PacketType::LapData),
        3 => Ok(PacketType::Event),
        4 => Ok(PacketType::Participants),
        5 => Ok(PacketType::CarSetups),
        6 => Ok(PacketType::CarTelemetry),
        7 => Ok(PacketType::CarStatus),
        8 => Ok(PacketType::FinalClassification),
        9 => Ok(PacketType::LobbyInfo),
        _ => Err(UnpackError(format!("Invalid PacketType: {}", value))),
    }
}
