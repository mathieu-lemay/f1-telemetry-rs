use std::io::BufRead;

use serde::Deserialize;

use crate::packet::event::*;
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::{assert_packet_size, seconds_to_millis, unpack_string};

use super::consts::*;

fn unpack_penalty_type(value: u8) -> Result<PenaltyType, UnpackError> {
    match value {
        0 => Ok(PenaltyType::DriveThrough),
        1 => Ok(PenaltyType::StopGo),
        2 => Ok(PenaltyType::GridPenalty),
        3 => Ok(PenaltyType::PenaltyReminder),
        4 => Ok(PenaltyType::TimePenalty),
        5 => Ok(PenaltyType::Warning),
        6 => Ok(PenaltyType::Disqualified),
        7 => Ok(PenaltyType::RemovedFromFormationLap),
        8 => Ok(PenaltyType::ParkedTooLongTimer),
        9 => Ok(PenaltyType::TyreRegulations),
        10 => Ok(PenaltyType::ThisLapInvalidated),
        11 => Ok(PenaltyType::ThisAndNextLapInvalidated),
        12 => Ok(PenaltyType::ThisLapInvalidatedWithoutReason),
        13 => Ok(PenaltyType::ThisAndNextLapInvalidatedWithoutReason),
        14 => Ok(PenaltyType::ThisAndPreviousLapInvalidated),
        15 => Ok(PenaltyType::ThisAndPreviousLapInvalidatedWithoutReason),
        16 => Ok(PenaltyType::Retired),
        17 => Ok(PenaltyType::BlackFlagTimer),
        _ => Err(UnpackError(format!("Invalid PenaltyType value: {}", value))),
    }
}

fn unpack_infringement_type(value: u8) -> Result<InfringementType, UnpackError> {
    match value {
        0 => Ok(InfringementType::BlockingBySlowDriving),
        1 => Ok(InfringementType::BlockingByWrongWayDriving),
        2 => Ok(InfringementType::ReversingOffTheStartLine),
        3 => Ok(InfringementType::BigCollision),
        4 => Ok(InfringementType::SmallCollision),
        5 => Ok(InfringementType::CollisionFailedToHandBackPositionSingle),
        6 => Ok(InfringementType::CollisionFailedToHandBackPositionMultiple),
        7 => Ok(InfringementType::CornerCuttingGainedTime),
        8 => Ok(InfringementType::CornerCuttingOvertakeSingle),
        9 => Ok(InfringementType::CornerCuttingOvertakeMultiple),
        10 => Ok(InfringementType::CrossedPitExitLane),
        11 => Ok(InfringementType::IgnoringBlueFlags),
        12 => Ok(InfringementType::IgnoringYellowFlags),
        13 => Ok(InfringementType::IgnoringDriveThrough),
        14 => Ok(InfringementType::TooManyDriveThroughs),
        15 => Ok(InfringementType::DriveThroughReminderServeWithinNLaps),
        16 => Ok(InfringementType::DriveThroughReminderServeThisLap),
        17 => Ok(InfringementType::PitLaneSpeeding),
        18 => Ok(InfringementType::ParkedForTooLong),
        19 => Ok(InfringementType::IgnoringTyreRegulations),
        20 => Ok(InfringementType::TooManyPenalties),
        21 => Ok(InfringementType::MultipleWarnings),
        22 => Ok(InfringementType::ApproachingDisqualification),
        23 => Ok(InfringementType::TyreRegulationsSelectSingle),
        24 => Ok(InfringementType::TyreRegulationsSelectMultiple),
        25 => Ok(InfringementType::LapInvalidatedCornerCutting),
        26 => Ok(InfringementType::LapInvalidatedRunningWide),
        27 => Ok(InfringementType::CornerCuttingRanWideGainedTimeMinor),
        28 => Ok(InfringementType::CornerCuttingRanWideGainedTimeSignificant),
        29 => Ok(InfringementType::CornerCuttingRanWideGainedTimeExtreme),
        30 => Ok(InfringementType::LapInvalidatedWallRiding),
        31 => Ok(InfringementType::LapInvalidatedFlashbackUsed),
        32 => Ok(InfringementType::LapInvalidatedResetToTrack),
        33 => Ok(InfringementType::BlockingThePitlane),
        34 => Ok(InfringementType::JumpStart),
        35 => Ok(InfringementType::SafetyCarToCarCollision),
        36 => Ok(InfringementType::SafetyCarIllegalOvertake),
        37 => Ok(InfringementType::SafetyCarExceedingAllowedPace),
        38 => Ok(InfringementType::VirtualSafetyCarExceedingAllowedPace),
        39 => Ok(InfringementType::FormationLapBelowAllowedSpeed),
        40 => Ok(InfringementType::RetiredMechanicalFailure),
        41 => Ok(InfringementType::RetiredTerminallyDamaged),
        42 => Ok(InfringementType::SafetyCarFallingTooFarBack),
        43 => Ok(InfringementType::BlackFlagTimer),
        44 => Ok(InfringementType::UnservedStopGoPenalty),
        45 => Ok(InfringementType::UnservedDriveThroughPenalty),
        46 => Ok(InfringementType::EngineComponentChange),
        47 => Ok(InfringementType::GearboxChange),
        48 => Ok(InfringementType::LeagueGridPenalty),
        49 => Ok(InfringementType::RetryPenalty),
        50 => Ok(InfringementType::IllegalTimeGain),
        51 => Ok(InfringementType::MandatoryPitstop),
        _ => Err(UnpackError(format!(
            "Invalid InfringementType value: {}",
            value
        ))),
    }
}

/// This packet gives details of events that happen during the course of a session.
///
/// Frequency: When the event occurs
/// Size: 35 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:         Header
/// event_code:     Event string code, see below
/// event_details:  Event details - should be interpreted differently
///                 for each type
/// ```
///
/// ### Event Codes
/// ```text
/// Event                   Code    Description
/// Session Started         SSTA    Sent when the session starts
/// Session Ended           SEND    Sent when the session ends
/// Fastest Lap             FTLP    When a driver achieves the fastest lap
/// Retirement              RTMT    When a driver retires
/// DRS enabled             DRSE    Race control have enabled DRS
/// DRS disabled            DRSD    Race control have disabled DRS
/// Team mate in pits       TMPT    Your team mate has entered the pits
/// Chequered flag          CHQF    The chequered flag has been waved
/// Race Winner             RCWN    The race winner is announced
/// Penalty Issued          PENA    A penalty has been issued
/// Speed Trap Triggered    SPTP    Speed trap has been triggered by fastest speed
/// ```
#[derive(Deserialize)]
struct RawEvent {
    event_code: [u8; 4],
}

/// ## Specification
/// ```text
/// vehicle_idx:    Vehicle index of car achieving fastest lap
/// lap_time:       Lap time is in seconds
/// ```
#[derive(Deserialize)]
struct FastestLapDetails {
    vehicle_idx: u8,
    lap_time: f32,
}

/// ## Specification
/// ```text
/// vehicle_idx:    Vehicle index of car retiring
/// ```
#[derive(Deserialize)]
struct RetirementDetails {
    vehicle_idx: u8,
}

/// ## Specification
/// ```text
/// vehicle_idx:    Vehicle index of team mate
/// ```
#[derive(Deserialize)]
struct TeamMateInPitsDetails {
    vehicle_idx: u8,
}

/// ## Specification
/// ```text
/// vehicle_idx:    Vehicle index of the race winner
/// ```
#[derive(Deserialize)]
struct RaceWinnerDetails {
    vehicle_idx: u8,
}

/// ## Specification
/// ```text
/// penalty_time:       Penalty type
/// infringment_type:   Infringement type
/// vehicle_idx:        Vehicle index of the car the penalty is applied to
/// other_vehicle_idx:  Vehicle index of the other car involved
/// time:               Time gained, or time spent doing action in seconds
/// lap_num:            Lap the penalty occurred on
/// places_gained:      Number of places gained by this
/// ```
///
/// ### Penalty Types
/// ```text
/// ID  Penalty meaning
/// 0   Drive through
/// 1   Stop Go
/// 2   Grid penalty
/// 3   Penalty reminder
/// 4   Time penalty
/// 5   Warning
/// 6   Disqualified
/// 7   Removed from formation lap
/// 8   Parked too long timer
/// 9   Tyre regulations
/// 10  This lap invalidated
/// 11  This and next lap invalidated
/// 12  This lap invalidated without reason
/// 13  This and next lap invalidated without reason
/// 14  This and previous lap invalidated
/// 15  This and previous lap invalidated without reason
/// 16  Retired
/// 17  Black flag timer
/// ```
///
/// ### Infringment Types
/// ```text
/// ID  Infringement meaning
/// 0   Blocking by slow driving
/// 1   Blocking by wrong way driving
/// 2   Reversing off the start line
/// 3   Big Collision
/// 4   Small Collision
/// 5   Collision failed to hand back position single
/// 6   Collision failed to hand back position multiple
/// 7   Corner cutting gained time
/// 8   Corner cutting overtake single
/// 9   Corner cutting overtake multiple
/// 10  Crossed pit exit lane
/// 11  Ignoring blue flags
/// 12  Ignoring yellow flags
/// 13  Ignoring drive through
/// 14  Too many drive throughs
/// 15  Drive through reminder serve within n laps
/// 16  Drive through reminder serve this lap
/// 17  Pit lane speeding
/// 18  Parked for too long
/// 19  Ignoring tyre regulations
/// 20  Too many penalties
/// 21  Multiple warnings
/// 22  Approaching disqualification
/// 23  Tyre regulations select single
/// 24  Tyre regulations select multiple
/// 25  Lap invalidated corner cutting
/// 26  Lap invalidated running wide
/// 27  Corner cutting ran wide gained time minor
/// 28  Corner cutting ran wide gained time significant
/// 29  Corner cutting ran wide gained time extreme
/// 30  Lap invalidated wall riding
/// 31  Lap invalidated flashback used
/// 32  Lap invalidated reset to track
/// 33  Blocking the pitlane
/// 34  Jump start
/// 35  Safety car to car collision
/// 36  Safety car illegal overtake
/// 37  Safety car exceeding allowed pace
/// 38  Virtual safety car exceeding allowed pace
/// 39  Formation lap below allowed speed
/// 40  Retired mechanical failure
/// 41  Retired terminally damaged
/// 42  Safety car falling too far back
/// 43  Black flag timer
/// 44  Unserved stop go penalty
/// 45  Unserved drive through penalty
/// 46  Engine component change
/// 47  Gearbox change
/// 48  League grid penalty
/// 49  Retry penalty
/// 50  Illegal time gain
/// 51  Mandatory pitstop
/// ```
#[derive(Deserialize)]
struct PenaltyDetails {
    penalty_type: u8,
    infringement_type: u8,
    vehicle_idx: u8,
    other_vehicle_idx: u8,
    time: u8,
    lap_num: u8,
    places_gained: u8,
}

/// ## Specification
/// ```text
/// vehicle_idx:    Vehicle index of the vehicle triggering speed trap
/// speed:          Top speed achieved in kilometres per hour
/// ```
#[derive(Deserialize)]
struct SpeedTrapDetails {
    vehicle_idx: u8,
    speed: f32,
}

pub(crate) fn parse_event_data<T: BufRead>(
    mut reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketEventData, UnpackError> {
    assert_packet_size(size, EVENT_PACKET_SIZE)?;

    let event: RawEvent = bincode::deserialize_from(&mut reader)?;

    let event_code = unpack_string(&event.event_code)?;

    let event = match event_code.as_str() {
        "SSTA" => Ok(Event::SessionStarted),
        "SEND" => Ok(Event::SessionEnded),
        "FTLP" => {
            let details: FastestLapDetails = bincode::deserialize_from(reader)?;

            let evt_detail = FastestLap {
                vehicle_idx: details.vehicle_idx,
                lap_time: seconds_to_millis(details.lap_time as f64),
            };
            Ok(Event::FastestLap(evt_detail))
        }
        "RTMT" => {
            let details: RetirementDetails = bincode::deserialize_from(reader)?;

            let evt_detail = Retirement {
                vehicle_idx: details.vehicle_idx,
            };
            Ok(Event::Retirement(evt_detail))
        }
        "DRSE" => Ok(Event::DRSEnabled),
        "DRSD" => Ok(Event::DRSDisabled),
        "TMPT" => {
            let details: TeamMateInPitsDetails = bincode::deserialize_from(reader)?;

            let evt_detail = TeamMateInPits {
                vehicle_idx: details.vehicle_idx,
            };
            Ok(Event::TeamMateInPits(evt_detail))
        }
        "CHQF" => Ok(Event::ChequeredFlag),
        "RCWN" => {
            let details: RaceWinnerDetails = bincode::deserialize_from(reader)?;

            let evt_detail = RaceWinner {
                vehicle_idx: details.vehicle_idx,
            };
            Ok(Event::RaceWinner(evt_detail))
        }
        "PENA" => {
            let details: PenaltyDetails = bincode::deserialize_from(reader)?;

            let penalty_type = unpack_penalty_type(details.penalty_type)?;
            let infringement_type = unpack_infringement_type(details.infringement_type)?;

            let evt_detail = Penalty {
                vehicle_idx: details.vehicle_idx,
                penalty_type,
                infringement_type,
                other_vehicle_idx: details.other_vehicle_idx,
                time: details.time,
                lap_num: details.lap_num,
                places_gained: details.places_gained,
            };
            Ok(Event::Penalty(evt_detail))
        }
        "SPTP" => {
            let details: SpeedTrapDetails = bincode::deserialize_from(reader)?;

            let evt_detail = SpeedTrap {
                vehicle_idx: details.vehicle_idx,
                speed: details.speed,
            };
            Ok(Event::SpeedTrap(evt_detail))
        }
        _ => Err(UnpackError(format!("Invalid Event Code: {}", event_code))),
    }?;

    Ok(PacketEventData { header, event })
}
