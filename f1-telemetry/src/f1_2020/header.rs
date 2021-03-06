use std::io::BufRead;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::{assert_packet_at_least_size, read_millis_f32};

use super::consts::*;

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
    let session_time = read_millis_f32(reader);
    let frame_identifier = reader.read_u32::<LittleEndian>().unwrap();
    let player_car_index = reader.read_u8().unwrap();
    let secondary_player_car_index = reader.read_u8().unwrap();

    Ok(PacketHeader::from_2020(
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
    ))
}
