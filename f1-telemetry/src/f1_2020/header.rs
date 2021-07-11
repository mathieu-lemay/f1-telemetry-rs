use std::io::BufRead;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::packet::header::PacketHeader;
use crate::packet::{PacketType, UnpackError};
use crate::utils::assert_packet_at_least_size;

use super::consts::*;

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

pub(crate) fn parse_header<T: BufRead>(
    reader: &mut T,
    size: usize,
) -> Result<PacketHeader, UnpackError> {
    assert_packet_at_least_size(size, HEADER_SIZE)?;

    let packet_format = reader.read_u16::<LittleEndian>().unwrap();
    let game_major_version = reader.read_u8().unwrap();
    let game_minor_version = reader.read_u8().unwrap();
    let packet_version = reader.read_u8().unwrap();
    let packet_id = reader.read_u8().unwrap();
    let session_uid = reader.read_u64::<LittleEndian>().unwrap();
    let session_time = reader.read_f32::<LittleEndian>().unwrap();
    let frame_identifier = reader.read_u32::<LittleEndian>().unwrap();
    let player_car_index = reader.read_u8().unwrap();
    let secondary_player_car_index = reader.read_u8().unwrap();

    let packet_type = parse_packet_type(packet_id)?;

    Ok(PacketHeader::new(
        packet_format,
        game_major_version,
        game_minor_version,
        packet_version,
        packet_type,
        session_uid,
        session_time,
        frame_identifier,
        player_car_index,
        Some(secondary_player_car_index),
    ))
}
