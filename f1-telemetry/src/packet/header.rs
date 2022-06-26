use serde::Serialize;

use crate::packet::PacketType;

/// The header for each of the UDP telemetry packets.
///
/// ## Specification
/// ```text
/// packet_format:              game year (ex. 2019)
/// game_major_version:         game major version - "x.00"
/// game_minor_version:         game minor version - "1.xX"
/// packet_version:             version of this packet type, all start from 1
/// packet_type:                identifier for the packet type
/// session_uid:                unique identifier for the session
/// session_time:               session timestamp
/// frame_identifier:           identifier for the frame the data was retrieved on
/// player_car_index:           index of player's car in the array
/// secondary_player_car_index: index of secondary player's car in the array (255 if no 2nd player)
/// ```
///
/// Possible `packet_type` values: [`PacketType`].
///
/// [`PacketType`]: ../enum.PacketType.html
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PacketHeader {
    pub packet_format: u16,
    pub game_major_version: u8,
    pub game_minor_version: u8,
    pub packet_version: u8,
    pub packet_type: PacketType,
    pub session_uid: u64,
    pub session_time: u32,
    pub frame_identifier: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: Option<u8>,
}
