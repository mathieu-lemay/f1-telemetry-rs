use byteorder::{LittleEndian, ReadBytesExt};
use std::io::BufRead;

use crate::packet::event::*;
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::{assert_packet_size, unpack_string};

const PACKET_SIZE: usize = 35;

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

pub(crate) fn parse_event_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketEventData, UnpackError> {
    assert_packet_size(size, PACKET_SIZE)?;

    let event_code = unpack_string(reader, 4)?;

    let event = match event_code.as_str() {
        "SSTA" => Ok(Event::SessionStarted),
        "SEND" => Ok(Event::SessionEnded),
        "FTLP" => {
            let vehicle_idx = reader.read_u8().unwrap();
            let lap_time = reader.read_f32::<LittleEndian>().unwrap();

            let evt_detail = FastestLap::new(vehicle_idx, lap_time);
            Ok(Event::FastestLap(evt_detail))
        }
        "RTMT" => {
            let vehicle_idx = reader.read_u8().unwrap();

            let evt_detail = Retirement::new(vehicle_idx);
            Ok(Event::Retirement(evt_detail))
        }
        "DRSE" => Ok(Event::DRSEnabled),
        "DRSD" => Ok(Event::DRSDisabled),
        "TMPT" => {
            let vehicle_idx = reader.read_u8().unwrap();

            let evt_detail = TeamMateInPits::new(vehicle_idx);
            Ok(Event::TeamMateInPits(evt_detail))
        }
        "CHQF" => Ok(Event::ChequeredFlag),
        "RCWN" => {
            let vehicle_idx = reader.read_u8().unwrap();

            let evt_detail = RaceWinner::new(vehicle_idx);
            Ok(Event::RaceWinner(evt_detail))
        }
        "PENA" => {
            let penalty_type = unpack_penalty_type(reader.read_u8().unwrap())?;
            let infringement_type = unpack_infringement_type(reader.read_u8().unwrap())?;
            let vehicle_idx = reader.read_u8().unwrap();
            let other_vehicle_idx = reader.read_u8().unwrap();
            let time = reader.read_u8().unwrap();
            let lap_num = reader.read_u8().unwrap();
            let places_gained = reader.read_u8().unwrap();

            let evt_detail = Penalty::new(
                vehicle_idx,
                penalty_type,
                infringement_type,
                other_vehicle_idx,
                time,
                lap_num,
                places_gained,
            );
            Ok(Event::Penalty(evt_detail))
        }
        "SPTP" => {
            let vehicle_idx = reader.read_u8().unwrap();
            let speed = reader.read_f32::<LittleEndian>().unwrap();

            let evt_detail = SpeedTrap::new(vehicle_idx, speed);
            Ok(Event::SpeedTrap(evt_detail))
        }
        _ => Err(UnpackError(format!("Invalid Event Code: {}", event_code))),
    }?;

    Ok(PacketEventData::new(header, event))
}

/*
*/
