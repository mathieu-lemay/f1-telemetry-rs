use byteorder::{LittleEndian, ReadBytesExt};
use getset::Getters;
use std::io::BufRead;

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
#[derive(Debug, Getters)]
pub struct PacketHeader {
    #[getset(get = "pub")]
    packet_format: u16,
    #[getset(get = "pub")]
    game_major_version: u8,
    #[getset(get = "pub")]
    game_minor_version: u8,
    #[getset(get = "pub")]
    packet_version: u8,
    #[getset(get = "pub")]
    packet_id: u8,
    #[getset(get = "pub")]
    session_uid: u64,
    #[getset(get = "pub")]
    session_time: f32,
    #[getset(get = "pub")]
    frame_identifier: u32,
    #[getset(get = "pub")]
    player_car_index: u8,
}

impl PacketHeader {
    pub fn new<T: BufRead>(reader: &mut T) -> PacketHeader {
        let packet_format = reader.read_u16::<LittleEndian>().unwrap();
        let game_major_version = reader.read_u8().unwrap();
        let game_minor_version = reader.read_u8().unwrap();
        let packet_version = reader.read_u8().unwrap();
        let packet_id = reader.read_u8().unwrap();
        let session_uid = reader.read_u64::<LittleEndian>().unwrap();
        let session_time = reader.read_f32::<LittleEndian>().unwrap();
        let frame_identifier = reader.read_u32::<LittleEndian>().unwrap();
        let player_car_index = reader.read_u8().unwrap();

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
        }
    }
}
