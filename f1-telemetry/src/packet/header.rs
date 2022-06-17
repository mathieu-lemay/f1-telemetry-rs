use getset::CopyGetters;

/// The header for each of the UDP telemetry packets.
///
/// ## Specification
/// ```text
/// packet_format:      2019
/// game_major_version: game major version - "x.00"
/// game_minor_version: game minor version - "1.xX"
/// packet_version:     version of this packet type, all start from 1
/// packet_id:          identifier for the packet type
/// session_uid:        unique identifier for the session
/// session_time:       session timestamp
/// frame_identifier:   identifier for the frame the data was retrieved on
/// player_car_index:   index of player's car in the array
/// ```
///
/// Possible `packet_id` values: [`PacketType`].
///
/// [`PacketType`]: ../enum.PacketType.html
#[derive(Debug, PartialEq, CopyGetters, Clone)]
#[getset(get_copy = "pub")]
pub struct PacketHeader {
    packet_format: u16,
    game_major_version: u8,
    game_minor_version: u8,
    packet_version: u8,
    packet_id: u8,
    session_uid: u64,
    session_time: u32,
    frame_identifier: u32,
    player_car_index: u8,
    secondary_player_car_index: u8,
}

#[allow(clippy::too_many_arguments)]
impl PacketHeader {
    pub(crate) fn from_2019(
        packet_format: u16,
        game_major_version: u8,
        game_minor_version: u8,
        packet_version: u8,
        packet_id: u8,
        session_uid: u64,
        session_time: u32,
        frame_identifier: u32,
        player_car_index: u8,
    ) -> PacketHeader {
        PacketHeader {
            packet_format,
            game_major_version,
            game_minor_version,
            packet_version,
            packet_id,
            session_uid,
            session_time,
            frame_identifier,
            player_car_index,
            secondary_player_car_index: 255,
        }
    }

    pub(crate) fn from_2020(
        packet_format: u16,
        game_major_version: u8,
        game_minor_version: u8,
        packet_version: u8,
        packet_id: u8,
        session_uid: u64,
        session_time: u32,
        frame_identifier: u32,
        player_car_index: u8,
        secondary_player_car_index: u8,
    ) -> PacketHeader {
        PacketHeader {
            packet_format,
            game_major_version,
            game_minor_version,
            packet_version,
            packet_id,
            session_uid,
            session_time,
            frame_identifier,
            player_car_index,
            secondary_player_car_index,
        }
    }
}
