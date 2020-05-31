use byteorder::{LittleEndian, ReadBytesExt};
use getset::{CopyGetters, Getters};
use std::convert::TryFrom;
use std::io::BufRead;

use super::header::PacketHeader;
use crate::packet::UnpackError;

/// List of possible events.
///
/// ## Specification
/// ```text
/// SessionStarted: Sent when the session starts
/// SessionEnded:   Sent when the session ends
/// FastestLap:     When a driver achieves the fastest lap
/// Retirement:     When a driver retires
/// DRSEnabled:     Race control have enabled DRS
/// DRSDisabled:    Race control have disabled DRS
/// TeamMateInPits: Your team mate has entered the pits
/// ChequeredFlag:  The chequered flag has been waved
/// RaceWinner:     The race winner is announced
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Event {
    SessionStarted,
    SessionEnded,
    FastestLap,
    Retirement,
    DRSEnabled,
    DRSDisabled,
    TeamMateInPits,
    ChequeredFlag,
    RaceWinner,
}

impl Event {
    pub fn description<'a>(self) -> &'a str {
        match self {
            Event::SessionStarted => "Session Started",
            Event::SessionEnded => "Session Ended",
            Event::FastestLap => "Fastest Lap",
            Event::Retirement => "Retirement",
            Event::DRSEnabled => "DRS Enabled",
            Event::DRSDisabled => "DRS Disabled",
            Event::TeamMateInPits => "Teammate In Pits",
            Event::ChequeredFlag => "Chequered Flag",
            Event::RaceWinner => "Race Winner",
        }
    }
}

impl TryFrom<&str> for Event {
    type Error = UnpackError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "SSTA" => Ok(Event::SessionStarted),
            "SEND" => Ok(Event::SessionEnded),
            "FTLP" => Ok(Event::FastestLap),
            "RTMT" => Ok(Event::Retirement),
            "DRSE" => Ok(Event::DRSEnabled),
            "DRSD" => Ok(Event::DRSDisabled),
            "TMPT" => Ok(Event::TeamMateInPits),
            "CHQF" => Ok(Event::ChequeredFlag),
            "RCWN" => Ok(Event::RaceWinner),
            _ => Err(UnpackError(format!("Invalid Event value: {}", value))),
        }
    }
}

/// This packet gives details of events that happen during the course of a session.
///
/// Frequency: When the event occurs
///
/// Size: 32 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// header:            Header
/// event_string_code: Event string code
///
/// # Event details - should be interpreted differently for each type
/// vehicle_idx:       Vehicle index of car (valid for events: FTLP, RTMT, TMPT, RCWN)
/// lap_time:          Lap time is in seconds (valid for events: FTLP)
/// ```
#[derive(Debug, CopyGetters, Getters)]
pub struct PacketEventData {
    #[getset(get = "pub")]
    header: PacketHeader,
    #[getset(get_copy = "pub")]
    event: Event,
    #[getset(get_copy = "pub")]
    vehicle_idx: Option<u8>,
    #[getset(get_copy = "pub")]
    lap_time: Option<f32>,
}

impl PacketEventData {
    pub fn new<T: BufRead>(
        reader: &mut T,
        header: PacketHeader,
    ) -> Result<PacketEventData, UnpackError> {
        let event = read_event(reader)?;

        let vehicle_idx = reader.read_u8().unwrap();
        let vehicle_idx = match event {
            Event::FastestLap | Event::Retirement | Event::TeamMateInPits | Event::RaceWinner => {
                Some(vehicle_idx)
            }
            _ => None,
        };

        let lap_time = reader.read_f32::<LittleEndian>().unwrap();
        let lap_time = match event {
            Event::FastestLap => Some(lap_time),
            _ => None,
        };

        Ok(PacketEventData {
            header,
            event,
            vehicle_idx,
            lap_time,
        })
    }
}

fn read_event<T: BufRead>(reader: &mut T) -> Result<Event, UnpackError> {
    let code_str: String = vec![
        reader.read_u8().unwrap() as char,
        reader.read_u8().unwrap() as char,
        reader.read_u8().unwrap() as char,
        reader.read_u8().unwrap() as char,
    ]
    .iter()
    .collect();

    Event::try_from(code_str.as_str())
}
