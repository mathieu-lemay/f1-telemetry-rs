use serde::Serialize;
use std::error::Error;
use std::fmt::{Display, Formatter};

use car_damage::PacketCarDamageData;
use car_setup::PacketCarSetupData;
use car_status::PacketCarStatusData;
use car_telemetry::PacketCarTelemetryData;
use event::PacketEventData;
use final_classification::PacketFinalClassificationData;
use header::PacketHeader;
use lap::PacketLapData;
use lobby_info::PacketLobbyInfoData;
use motion::PacketMotionData;
use motion_ex::PacketMotionExData;
use participants::PacketParticipantsData;
use session::PacketSessionData;
use session_history::PacketSessionHistoryData;
use tyre_sets::PacketTyreSetsData;

use super::{f1_2019, f1_2020, f1_2021, f1_2022, f1_2023};

pub mod car_damage;
pub mod car_setup;
pub mod car_status;
pub mod car_telemetry;
pub mod event;
pub mod final_classification;
pub mod generic;
pub mod header;
pub mod lap;
pub mod lobby_info;
pub mod motion;
pub mod motion_ex;
pub mod participants;
pub mod session;
pub mod session_history;
pub mod tyre_sets;

#[derive(Debug, Eq, PartialEq)]
pub struct UnpackError(pub String);

impl From<Box<bincode::ErrorKind>> for UnpackError {
    fn from(e: Box<bincode::ErrorKind>) -> Self {
        UnpackError(e.to_string())
    }
}

impl Display for UnpackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&format!("Unpack error: {}", &self.0), f)
    }
}

impl Error for UnpackError {}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "packet_type")]
pub enum Packet {
    Motion(PacketMotionData),
    Session(PacketSessionData),
    LapData(PacketLapData),
    Event(PacketEventData),
    Participants(PacketParticipantsData),
    CarSetups(PacketCarSetupData),
    CarTelemetry(PacketCarTelemetryData),
    CarStatus(PacketCarStatusData),
    FinalClassification(PacketFinalClassificationData),
    LobbyInfo(PacketLobbyInfoData),
    CarDamage(PacketCarDamageData),
    SessionHistory(PacketSessionHistoryData),
    TyreSets(PacketTyreSetsData),
    MotionEx(PacketMotionExData),
}

impl Packet {
    pub fn header(&self) -> &PacketHeader {
        match self {
            Packet::Motion(p) => &p.header,
            Packet::Session(p) => &p.header,
            Packet::LapData(p) => &p.header,
            Packet::Event(p) => &p.header,
            Packet::Participants(p) => &p.header,
            Packet::CarSetups(p) => &p.header,
            Packet::CarTelemetry(p) => &p.header,
            Packet::CarStatus(p) => &p.header,
            Packet::FinalClassification(p) => &p.header,
            Packet::LobbyInfo(p) => &p.header,
            Packet::CarDamage(p) => &p.header,
            Packet::SessionHistory(p) => &p.header,
            Packet::TyreSets(p) => &p.header,
            Packet::MotionEx(p) => &p.header,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Eq, PartialEq)]
pub enum PacketType {
    Motion,
    Session,
    LapData,
    Event,
    Participants,
    CarSetups,
    CarTelemetry,
    CarStatus,
    FinalClassification,
    LobbyInfo,
    CarDamage,
    SessionHistory,
    TyreSets,
    MotionEx,
}

impl From<PacketType> for u8 {
    fn from(val: PacketType) -> u8 {
        match val {
            PacketType::Motion => 0,
            PacketType::Session => 1,
            PacketType::LapData => 2,
            PacketType::Event => 3,
            PacketType::Participants => 4,
            PacketType::CarSetups => 5,
            PacketType::CarTelemetry => 6,
            PacketType::CarStatus => 7,
            PacketType::FinalClassification => 8,
            PacketType::LobbyInfo => 9,
            PacketType::CarDamage => 10,
            PacketType::SessionHistory => 11,
            PacketType::TyreSets => 12,
            PacketType::MotionEx => 13,
        }
    }
}

pub fn parse_packet(size: usize, packet: &[u8]) -> Result<Packet, UnpackError> {
    let packet_format = parse_version(packet);

    match packet_format {
        2019 => Ok(f1_2019::parse_packet(size, packet)?),
        2020 => Ok(f1_2020::parse_packet(size, packet)?),
        2021 => Ok(f1_2021::parse_packet(size, packet)?),
        2022 => Ok(f1_2022::parse_packet(size, packet)?),
        2023 => Ok(f1_2023::parse_packet(size, packet)?),
        _ => Err(UnpackError(format!(
            "Invalid packet: unknown format ({})",
            packet_format
        ))),
    }
}

fn parse_version(packet: &[u8]) -> u16 {
    packet[0] as u16 | ((packet[1] as u16) << 8)
}
