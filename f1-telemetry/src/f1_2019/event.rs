use std::io::BufRead;

use serde::Deserialize;

use crate::packet::event::*;
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::{assert_packet_size, seconds_to_millis, unpack_string};

use super::consts::*;

/// This packet gives details of events that happen during the course of a session.
///
/// Frequency: When the event occurs
/// Size: 32 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:         Header
/// event_code:     Event string code, see below
/// vehicle_idx:    Index of the event's associated vehicle (if applicable)
/// lap_time:       Lap time is in seconds (if applicable)
/// ```
///
/// ### Event Codes
/// ```text
/// Event               Code    Description
/// Session Started     SSTA    Sent when the session starts
/// Session Ended       SEND    Sent when the session ends
/// Fastest Lap         FTLP    When a driver achieves the fastest lap
/// Retirement          RTMT    When a driver retires
/// DRS enabled         DRSE    Race control have enabled DRS
/// DRS disabled        DRSD    Race control have disabled DRS
/// Team mate in pits   TMPT    Your team mate has entered the pits
/// Chequered flag      CHQF    The chequered flag has been waved
/// Race Winner         RCWN    The race winner is announced
/// ```
#[derive(Deserialize)]
struct RawEvent {
    event_code: [u8; 4],
    vehicle_idx: u8,
    lap_time: f32,
}

pub(crate) fn parse_event_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketEventData, UnpackError> {
    assert_packet_size(size, EVENT_PACKET_SIZE)?;

    let event: RawEvent = bincode::deserialize_from(reader)?;

    let event_code = unpack_string(&event.event_code)?;

    let event = match event_code.as_str() {
        "SSTA" => Ok(Event::SessionStarted),
        "SEND" => Ok(Event::SessionEnded),
        "FTLP" => {
            let evt_detail = FastestLap {
                vehicle_idx: event.vehicle_idx,
                lap_time: seconds_to_millis(event.lap_time as f64),
            };
            Ok(Event::FastestLap(evt_detail))
        }
        "RTMT" => {
            let evt_detail = Retirement {
                vehicle_idx: event.vehicle_idx,
            };
            Ok(Event::Retirement(evt_detail))
        }
        "DRSE" => Ok(Event::DRSEnabled),
        "DRSD" => Ok(Event::DRSDisabled),
        "TMPT" => {
            let evt_detail = TeamMateInPits {
                vehicle_idx: event.vehicle_idx,
            };
            Ok(Event::TeamMateInPits(evt_detail))
        }
        "CHQF" => Ok(Event::ChequeredFlag),
        "RCWN" => {
            let evt_detail = RaceWinner {
                vehicle_idx: event.vehicle_idx,
            };
            Ok(Event::RaceWinner(evt_detail))
        }
        _ => Err(UnpackError(format!("Invalid Event Code: {}", event_code))),
    }?;

    Ok(PacketEventData { header, event })
}
