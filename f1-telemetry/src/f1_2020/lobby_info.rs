use std::io::BufRead;

use serde::Deserialize;

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
///
/// This packet details the players currently in a multiplayer lobby. It details each player's
/// selected car, any AI involved in the game and also the ready status of each of the participants.
///
/// Frequency: Two every second when in the lobby
/// Size: 1169 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:      Header
/// num_players: Number of players in the lobby data
/// players:     List of Players
/// ```
#[derive(Deserialize)]
struct RawLobbyInfo {
    num_players: u8,
    players: [RawPlayer; NUMBER_CARS],
}

/// This type is used for the `players` array of the [`RawLobbyInfoData`] type.
///
/// ## Specification
/// ```text
/// ai_controlled: Whether the vehicle is AI or Human.
/// team:          Team of the player
/// nationality:   Nationality of the player
/// name:          Name of participant in UTF-8 format – null terminated
/// ready_status:  Player's ready status
/// ```
#[derive(Deserialize)]
struct RawPlayer {
    ai_controlled: bool,
    team: u8,
    nationality: u8,
    name1: [u8; 32], // FIXME: Ugly hack
    name2: [u8; 16],
    ready_status: u8,
}

impl Player {
    fn from_2020(player: &RawPlayer) -> Result<Self, UnpackError> {
        let name: [u8; 48] = {
            let mut whole: [u8; 48] = [0; 48];
            let (part1, part2) = whole.split_at_mut(player.name1.len());
            part1.copy_from_slice(&player.name1);
            part2.copy_from_slice(&player.name2);
            whole
        };

        let team = unpack_team(player.team)?;
        let nationality = unpack_nationality(player.nationality)?;
        let name = unpack_string(&name)?;
        let ready_status = unpack_ready_status(player.ready_status)?;

        Ok(Player {
            ai_controlled: player.ai_controlled,
            team,
            nationality,
            name,
            ready_status,
        })
    }
}

pub(crate) fn parse_lobby_info_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketLobbyInfoData, UnpackError> {
    assert_packet_size(size, LOBBY_INFO_PACKET_SIZE)?;

    let lobby_info: RawLobbyInfo = bincode::deserialize_from(reader)?;

    let players = lobby_info
        .players
        .iter()
        .map(Player::from_2020)
        .collect::<Result<Vec<Player>, UnpackError>>()?;

    Ok(PacketLobbyInfoData {
        header,
        num_players: lobby_info.num_players,
        players,
    })
}
