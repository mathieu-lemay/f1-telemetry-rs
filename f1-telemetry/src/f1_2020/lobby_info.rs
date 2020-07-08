use std::io::BufRead;

use byteorder::ReadBytesExt;

use crate::f1_2020::generic::{unpack_nationality, unpack_team};
use crate::packet::header::PacketHeader;
use crate::packet::lobby_info::{PacketLobbyInfoData, Player, ReadyStatus};
use crate::packet::UnpackError;
use crate::utils::{assert_packet_size, unpack_string};

use super::consts::*;

fn unpack_ready_status(value: u8) -> Result<ReadyStatus, UnpackError> {
    match value {
        0 => Ok(ReadyStatus::NotReady),
        1 => Ok(ReadyStatus::Ready),
        2 => Ok(ReadyStatus::Spectating),
        _ => Err(UnpackError(format!("Invalid ReadyStatus value: {}", value))),
    }
}

fn parse_player<T: BufRead>(reader: &mut T) -> Result<Player, UnpackError> {
    let ai_controlled = reader.read_u8().unwrap() == 1;
    let team = unpack_team(reader.read_u8().unwrap())?;
    let nationality = unpack_nationality(reader.read_u8().unwrap())?;
    let name = unpack_string(reader, 48)?;
    let ready_status = unpack_ready_status(reader.read_u8().unwrap())?;

    Ok(Player::new(
        ai_controlled,
        team,
        nationality,
        name,
        ready_status,
    ))
}

pub(crate) fn parse_lobby_info_data<T: BufRead>(
    mut reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketLobbyInfoData, UnpackError> {
    assert_packet_size(size, LOBBY_INFO_PACKET_SIZE)?;

    let num_players = reader.read_u8().unwrap();

    let mut players = Vec::with_capacity(NUMBER_CARS);

    for _ in 0..NUMBER_CARS {
        let p = parse_player(&mut reader)?;
        players.push(p);
    }

    Ok(PacketLobbyInfoData::new(header, num_players, players))
}
