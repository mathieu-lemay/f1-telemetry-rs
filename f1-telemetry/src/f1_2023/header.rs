use std::io::BufRead;

use serde::Deserialize;

use crate::packet::header::PacketHeader;
use crate::packet::{PacketType, UnpackError};
use crate::utils::{assert_packet_at_least_size, seconds_to_millis};

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
        10 => Ok(PacketType::CarDamage),
        11 => Ok(PacketType::SessionHistory),
        12 => Ok(PacketType::TyreSets),
        //13 => Ok(PacketType::MotionEx),
        _ => Err(UnpackError(format!("Invalid PacketType: {}", value))),
    }
}

/// The header for each of the UDP telemetry packets.
///
/// ## Specification
/// ```text
/// packet_format:              2023
/// game_year:                  game year - last two digits (e.g. 23)
/// game_major_version:         game major version - "x.00"
/// game_minor_version:         game minor version - "1.xx"
/// packet_version:             version of this packet type, all start from 1
/// packet_type:                identifier for the packet type
/// session_uid:                unique identifier for the session
/// session_time:               session timestamp
/// frame_identifier:           identifier for the frame the data was retrieved on
/// overall_frame_identifier:   Overall identifier for the frame the data was retrieved
///                             on, doesn't go back after flashbacks
/// player_car_index:           index of player's car in the array
/// secondary_player_car_index: index of secondary player's car in the array (255 if no 2nd player)
/// ```
///
/// ### Packet Types
/// ```text
/// Packet Name             Value   Description
/// Motion                  0       Contains all motion data for player’s car – only sent while player is
///                                 in control
/// Session                 1       Data about the session – track, time left
/// Lap Data                2       Data about all the lap times of cars in the session
/// Event                   3       Various notable events that happen during a session
/// Participants            4       List of participants in the session, mostly relevant for multiplayer
/// Car Setups              5       Packet detailing car setups for cars in the race
/// Car Telemetry           6       Telemetry data for all cars
/// Car Status              7       Status data for all cars such as damage
/// Final Classification    8       Final classification confirmation at the end of a race
/// Lobby Info              9       Information about players in a multiplayer lobby
/// Car Damage              10      Damage status for all cars
/// Session History         11      Lap and tyre data for session
/// Tyre Sets               12      Extended tyre set data
/// Motion Ex               13      Extended motion data for player car
/// ```
#[derive(Deserialize)]
struct Header {
    packet_format: u16,
    game_year: u8,
    game_major_version: u8,
    game_minor_version: u8,
    packet_version: u8,
    packet_id: u8,
    session_uid: u64,
    session_time: f32,
    frame_identifier: u32,
    overall_frame_identifier: u32,
    player_car_index: u8,
    secondary_player_car_index: u8,
}

impl TryFrom<Header> for PacketHeader {
    type Error = UnpackError;

    fn try_from(header: Header) -> Result<Self, Self::Error> {
        let packet_type = parse_packet_type(header.packet_id)?;
        let session_time = seconds_to_millis(header.session_time as f64);
        let secondary_player_car_index = match header.secondary_player_car_index {
            255 => None,
            idx => Some(idx),
        };

        Ok(Self {
            packet_format: header.packet_format,
            game_year: header.game_year,
            game_major_version: header.game_major_version,
            game_minor_version: header.game_minor_version,
            packet_version: header.packet_version,
            packet_type,
            session_uid: header.session_uid,
            session_time,
            frame_identifier: header.frame_identifier,
            overall_frame_identifier: Some(header.overall_frame_identifier),
            player_car_index: header.player_car_index,
            secondary_player_car_index,
        })
    }
}

pub(crate) fn parse_header<T: BufRead>(
    reader: &mut T,
    size: usize,
) -> Result<PacketHeader, UnpackError> {
    assert_packet_at_least_size(size, HEADER_SIZE)?;

    let header: Header = bincode::deserialize_from(reader)?;

    header.try_into()
}
