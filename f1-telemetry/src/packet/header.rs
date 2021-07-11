use getset::CopyGetters;

use crate::packet::PacketType;
use crate::utils::seconds_to_millis;

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
#[derive(Debug, PartialEq, CopyGetters, Clone)]
#[getset(get_copy = "pub")]
pub struct PacketHeader {
    packet_format: u16,
    game_major_version: u8,
    game_minor_version: u8,
    packet_version: u8,
    packet_type: PacketType,
    session_uid: u64,
    session_time: u32,
    frame_identifier: u32,
    player_car_index: u8,
    secondary_player_car_index: Option<u8>,
}

#[allow(clippy::too_many_arguments)]
impl PacketHeader {
    pub(crate) fn new(
        packet_format: u16,
        game_major_version: u8,
        game_minor_version: u8,
        packet_version: u8,
        packet_id: PacketType,
        session_uid: u64,
        session_time: f32,
        frame_identifier: u32,
        player_car_index: u8,
        secondary_player_car_index: Option<u8>,
    ) -> Self {
        Self {
            packet_format,
            game_major_version,
            game_minor_version,
            packet_version,
            packet_type: packet_id,
            session_uid,
            session_time: seconds_to_millis(session_time as f64),
            frame_identifier,
            player_car_index,
            secondary_player_car_index,
        }
    }
}
