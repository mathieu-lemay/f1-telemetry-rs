use std::convert::TryFrom;
use std::io::Cursor;

use crate::f1_2019::car_setup::parse_car_setup_data;
use crate::f1_2019::car_status::parse_car_status_data;
use crate::f1_2019::car_telemetry::parse_car_telemetry_data;
use crate::f1_2019::event::parse_event_data;
use crate::f1_2019::header::parse_header;
use crate::f1_2019::lap::parse_lap_data;
use crate::f1_2019::motion::parse_motion_data;
use crate::f1_2019::participants::parse_participants_data;
use crate::f1_2019::session::parse_session_data;
use crate::packet::{Packet, PacketType, UnpackError};

mod car_setup;
mod car_status;
mod car_telemetry;
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

    let packet_id: PacketType = PacketType::try_from(header.packet_id())?;

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
        p => Err(UnpackError(format!("Unsupported packet type: {:?}", p))),
    }
}

impl TryFrom<u8> for PacketType {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketType::Motion),
            1 => Ok(PacketType::Session),
            2 => Ok(PacketType::LapData),
            3 => Ok(PacketType::Event),
            4 => Ok(PacketType::Participants),
            5 => Ok(PacketType::CarSetups),
            6 => Ok(PacketType::CarTelemetry),
            7 => Ok(PacketType::CarStatus),
            _ => Err(UnpackError(format!("Invalid PacketType: {}", value))),
        }
    }
}
