use std::io::BufRead;

use serde::Deserialize;

use crate::packet::header::PacketHeader;
use crate::packet::{PacketType, UnpackError};
use crate::utils::{assert_packet_at_least_size, seconds_to_millis};

use super::consts::*;

fn parse_packet_type(value: u8) -> Result<PacketType, UnpackError> {
    Err(UnpackError(format!("Invalid PacketType: {}", value)))
}

/// The header for each of the UDP telemetry packets.
///
/// ## Specification
/// ```text
/// packet_format:              game year (ex. 2021)
/// game_major_version:         game major version - "x.00"
/// game_minor_version:         game minor version - "1.xx"
/// packet_version:             version of this packet type, all start from 1
/// packet_type:                identifier for the packet type
/// session_uid:                unique identifier for the session
/// session_time:               session timestamp
/// frame_identifier:           identifier for the frame the data was retrieved on
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
/// ```
#[derive(Deserialize)]
struct Header {
    packet_format: u16,
    game_major_version: u8,
    game_minor_version: u8,
    packet_version: u8,
    packet_id: u8,
    session_uid: u64,
    session_time: f32,
    frame_identifier: u32,
    player_car_index: u8,
    secondary_player_car_index: u8,
}

impl PacketHeader {
    fn from_2021(header: &Header) -> Result<Self, UnpackError> {
        let packet_type = parse_packet_type(header.packet_id)?;

        let session_time = seconds_to_millis(header.session_time as f64);

        Ok(PacketHeader {
            packet_format: header.packet_format,
            game_major_version: header.game_major_version,
            game_minor_version: header.game_minor_version,
            packet_version: header.packet_version,
            packet_type,
            session_uid: header.session_uid,
            session_time,
            frame_identifier: header.frame_identifier,
            player_car_index: header.player_car_index,
            secondary_player_car_index: Some(header.secondary_player_car_index),
        })
    }
}

pub(crate) fn parse_header<T: BufRead>(
    reader: &mut T,
    size: usize,
) -> Result<PacketHeader, UnpackError> {
    assert_packet_at_least_size(size, HEADER_SIZE)?;

    let header: Header = bincode::deserialize_from(reader)?;

    PacketHeader::from_2021(&header)
}