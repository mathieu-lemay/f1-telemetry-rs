use getset::{CopyGetters, Getters};

use crate::packet::generic::{Nationality, Team};

use super::header::PacketHeader;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ReadyStatus {
    NotReady,
    Ready,
    Spectating,
}

/// This type is used for the `players` array of the [`PacketLobbyInfoData`] type.
///
/// ## Specification
/// ```text
/// ai_controlled: Whether the vehicle is AI or Human.
/// team:          Team of the player
/// nationality:   Nationality of the player
/// name:          Name of participant in UTF-8 format – null terminated
/// ready_status:  Player's ready status
/// ```
#[derive(Debug, PartialEq, Getters, CopyGetters)]
pub struct Player {
    #[getset(get_copy = "pub")]
    ai_controlled: bool,
    #[getset(get_copy = "pub")]
    team: Team,
    #[getset(get_copy = "pub")]
    nationality: Nationality,
    #[getset(get = "pub")]
    name: String,
    #[getset(get_copy = "pub")]
    ready_status: ReadyStatus,
}

impl Player {
    pub fn new(
        ai_controlled: bool,
        team: Team,
        nationality: Nationality,
        name: String,
        ready_status: ReadyStatus,
    ) -> Self {
        Self {
            ai_controlled,
            team,
            nationality,
            name,
            ready_status,
        }
    }
}

/// This packet details the players currently in a multiplayer lobby. It details each player's
/// selected car, any AI involved in the game and also the ready status of each of the participants.
///
/// Frequency: Two every second when in the lobby
///
/// ## Specification
/// ```text
/// header:      Header
/// num_players: Number of players in the lobby data
/// players:     List of Players
/// ```
#[derive(Debug, PartialEq, CopyGetters, Getters)]
pub struct PacketLobbyInfoData {
    #[getset(get = "pub")]
    header: PacketHeader,
    #[getset(get_copy = "pub")]
    num_players: u8,
    #[getset(get = "pub")]
    players: Vec<Player>,
}

impl PacketLobbyInfoData {
    pub fn new(header: PacketHeader, num_players: u8, players: Vec<Player>) -> Self {
        Self {
            header,
            num_players,
            players,
        }
    }
}
