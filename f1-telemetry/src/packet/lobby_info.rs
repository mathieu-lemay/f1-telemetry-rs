use serde::Serialize;

use crate::packet::generic::{Nationality, Team};

use super::header::PacketHeader;

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize)]
pub enum ReadyStatus {
    #[default]
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
/// car_number:    Car number of the player
/// ready_status:  Player's ready status
/// ```
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize)]
pub struct Player {
    pub ai_controlled: bool,
    pub team: Team,
    pub nationality: Nationality,
    pub name: String,
    pub car_number: Option<u8>,
    pub ready_status: ReadyStatus,
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
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct PacketLobbyInfoData {
    pub header: PacketHeader,
    pub num_players: u8,
    pub players: Vec<Player>,
}
