use std::io::Cursor;

use car_setup::parse_car_setup_data;
use car_status::parse_car_status_data;
use car_telemetry::parse_car_telemetry_data;
use event::parse_event_data;
use final_classification::parse_final_classification_data;
use header::parse_header;
use lap::parse_lap_data;
use lobby_info::parse_lobby_info_data;
use motion::parse_motion_data;
use participants::parse_participants_data;
use session::parse_session_data;

use crate::packet::{Packet, PacketType, UnpackError};

mod car_setup;
mod car_status;
mod car_telemetry;
mod consts;
mod event;
mod final_classification;
mod generic;
mod header;
mod lap;
mod lobby_info;
mod motion;
mod participants;
mod session;

pub(crate) fn parse_packet(size: usize, packet: &[u8]) -> Result<Packet, UnpackError> {
    let mut cursor = Cursor::new(packet);
    let header = parse_header(&mut cursor, size)?;

    let packet_id: PacketType = parse_packet_type(header.packet_id())?;

    match packet_id {
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
        PacketType::CarSetups => {
            let packet = parse_car_setup_data(&mut cursor, header, size)?;

            Ok(Packet::CarSetups(packet))
        }
        PacketType::CarTelemetry => {
            let packet = parse_car_telemetry_data(&mut cursor, header, size)?;

            Ok(Packet::CarTelemetry(packet))
        }
        PacketType::CarStatus => {
            let packet = parse_car_status_data(&mut cursor, header, size)?;

            Ok(Packet::CarStatus(packet))
        }
        PacketType::FinalClassification => {
            let packet = parse_final_classification_data(&mut cursor, header, size)?;

            Ok(Packet::FinalClassification(packet))
        }
        PacketType::LobbyInfo => {
            let packet = parse_lobby_info_data(&mut cursor, header, size)?;

            Ok(Packet::LobbyInfo(packet))
        }
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
