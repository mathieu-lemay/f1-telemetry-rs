use getset::{CopyGetters, Getters};

use super::header::PacketHeader;

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
    pub(crate) fn new(
        header: PacketHeader,
        event: Event,
        vehicle_idx: Option<u8>,
        lap_time: Option<f32>,
    ) -> PacketEventData {
        PacketEventData {
            header,
            event,
            vehicle_idx,
            lap_time,
        }
    }
}
