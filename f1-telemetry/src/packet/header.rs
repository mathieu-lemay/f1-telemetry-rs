use serde::Serialize;

use crate::packet::PacketType;

/// The header for each of the UDP telemetry packets.
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct PacketHeader {
    /// Game year (ex. 2019)
    pub packet_format: u16,
    /// Game major version - "x.00"
    pub game_major_version: u8,
    /// Game minor version - "1.xX"
    pub game_minor_version: u8,
    /// Version of this packet type, all start from 1
    pub packet_version: u8,
    /// Type of packet
    pub packet_type: PacketType,
    /// Unique identifier for the session
    pub session_uid: u64,
    /// Session timestamp, in milliseconds
    pub session_time: u32,
    /// Identifier for the frame the data was retrieved on
    pub frame_identifier: u32,
    /// Index of player's car in the array
    pub player_car_index: u8,
    /// Index of secondary player's car in the array, if any
    pub secondary_player_car_index: Option<u8>,
}
