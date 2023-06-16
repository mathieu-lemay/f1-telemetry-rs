use std::io::BufRead;

use serde::Deserialize;

use crate::packet::header::PacketHeader;
use crate::packet::lobby_info::{PacketLobbyInfoData, Platform, Player, ReadyStatus};
use crate::packet::UnpackError;
use crate::utils::{assert_packet_size, unpack_string};

use super::consts::*;
use super::generic::{unpack_nationality, unpack_team};

fn unpack_ready_status(value: u8) -> Result<ReadyStatus, UnpackError> {
    match value {
        0 => Ok(ReadyStatus::NotReady),
        1 => Ok(ReadyStatus::Ready),
        2 => Ok(ReadyStatus::Spectating),
        _ => Err(UnpackError(format!("Invalid ReadyStatus value: {}", value))),
    }
}

pub(crate) fn unpack_platform(value: u8) -> Result<Platform, UnpackError> {
    match value {
        1 => Ok(Platform::Steam),
        3 => Ok(Platform::PlayStation),
        4 => Ok(Platform::Xbox),
        6 => Ok(Platform::Origin),
        255 => Ok(Platform::Unknown),
        _ => Err(UnpackError(format!("Invalid Platform value: {}", value))),
    }
}

///
/// This packet details the players currently in a multiplayer lobby. It details each player's
/// selected car, any AI involved in the game and also the ready status of each of the participants.
///
/// Frequency: Two every second when in the lobby
/// Size: 1218 bytes
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
/// ai_controlled: Whether the vehicle is AI (1) or Human (0) controlled
/// team_id:       Team id - see appendix (255 if no team currently selected)
/// nationality:   Nationality of the player
/// platform:      1 = Steam, 3 = PlayStation, 4 = Xbox, 6 = Origin, 255 = unknown
/// name:          Name of participant in UTF-8 format – null terminated
///                Will be truncated with ... (U+2026) if too long
/// car_number:    Car number of the player
/// ready_status:  0 = not ready, 1 = ready, 2 = spectating
/// ```
#[derive(Deserialize)]
struct RawPlayer {
    ai_controlled: bool,
    team_id: u8,
    nationality: u8,
    platform: u8,
    name1: [u8; 32], // FIXME: Ugly hack
    name2: [u8; 16],
    car_number: u8,
    ready_status: u8,
}

impl TryFrom<&RawPlayer> for Player {
    type Error = UnpackError;

    fn try_from(player: &RawPlayer) -> Result<Self, Self::Error> {
        let name: [u8; 48] = {
            let mut whole: [u8; 48] = [0; 48];
            let (part1, part2) = whole.split_at_mut(player.name1.len());
            part1.copy_from_slice(&player.name1);
            part2.copy_from_slice(&player.name2);
            whole
        };

        let team = unpack_team(player.team_id)?;
        let nationality = unpack_nationality(player.nationality)?;
        let platform = unpack_platform(player.platform)?;
        let name = unpack_string(&name)?;
        let ready_status = unpack_ready_status(player.ready_status)?;

        Ok(Self {
            ai_controlled: player.ai_controlled,
            team,
            nationality,
            platform,
            name,
            car_number: Some(player.car_number),
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
        .map(|p| p.try_into())
        .collect::<Result<Vec<Player>, UnpackError>>()?;

    Ok(PacketLobbyInfoData {
        header,
        num_players: lobby_info.num_players,
        players,
    })
}
