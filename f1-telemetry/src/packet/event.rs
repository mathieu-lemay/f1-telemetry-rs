use serde::Serialize;

use super::header::PacketHeader;

/// Description of a fastest lap event
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct FastestLap {
    /// Index of the vehicle that did the fastest lap
    pub vehicle_idx: u8,
    /// Lap time, in milliseconds
    pub lap_time: u32,
}

/// Description of a retirement event
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct Retirement {
    /// Index of the vehicle that retired
    pub vehicle_idx: u8,
}

/// Description of a teammate in pits event
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct TeamMateInPits {
    /// Index of the teammate's vehicle
    pub vehicle_idx: u8,
}

/// Description of a race winner event
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct RaceWinner {
    /// Index of the vehicle that won the race
    pub vehicle_idx: u8,
}

/// List of possible penalties
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub enum PenaltyType {
    DriveThrough,
    StopGo,
    GridPenalty,
    PenaltyReminder,
    TimePenalty,
    Warning,
    Disqualified,
    RemovedFromFormationLap,
    ParkedTooLongTimer,
    TyreRegulations,
    ThisLapInvalidated,
    ThisAndNextLapInvalidated,
    ThisLapInvalidatedWithoutReason,
    ThisAndNextLapInvalidatedWithoutReason,
    ThisAndPreviousLapInvalidated,
    ThisAndPreviousLapInvalidatedWithoutReason,
    Retired,
    BlackFlagTimer,
}

/// List of possible infringments
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub enum InfringementType {
    BlockingBySlowDriving,
    BlockingByWrongWayDriving,
    ReversingOffTheStartLine,
    BigCollision,
    SmallCollision,
    CollisionFailedToHandBackPositionSingle,
    CollisionFailedToHandBackPositionMultiple,
    CornerCuttingGainedTime,
    CornerCuttingOvertakeSingle,
    CornerCuttingOvertakeMultiple,
    CrossedPitExitLane,
    IgnoringBlueFlags,
    IgnoringYellowFlags,
    IgnoringDriveThrough,
    TooManyDriveThroughs,
    DriveThroughReminderServeWithinNLaps,
    DriveThroughReminderServeThisLap,
    PitLaneSpeeding,
    ParkedForTooLong,
    IgnoringTyreRegulations,
    TooManyPenalties,
    MultipleWarnings,
    ApproachingDisqualification,
    TyreRegulationsSelectSingle,
    TyreRegulationsSelectMultiple,
    LapInvalidatedCornerCutting,
    LapInvalidatedRunningWide,
    CornerCuttingRanWideGainedTimeMinor,
    CornerCuttingRanWideGainedTimeSignificant,
    CornerCuttingRanWideGainedTimeExtreme,
    LapInvalidatedWallRiding,
    LapInvalidatedFlashbackUsed,
    LapInvalidatedResetToTrack,
    BlockingThePitlane,
    JumpStart,
    SafetyCarToCarCollision,
    SafetyCarIllegalOvertake,
    SafetyCarExceedingAllowedPace,
    VirtualSafetyCarExceedingAllowedPace,
    FormationLapBelowAllowedSpeed,
    RetiredMechanicalFailure,
    RetiredTerminallyDamaged,
    SafetyCarFallingTooFarBack,
    BlackFlagTimer,
    UnservedStopGoPenalty,
    UnservedDriveThroughPenalty,
    EngineComponentChange,
    GearboxChange,
    LeagueGridPenalty,
    RetryPenalty,
    IllegalTimeGain,
    MandatoryPitstop,
    FormationLapParking,
    ParcFermeChange,
    AttributeAssigned,
}

/// Description of a penalty event
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct Penalty {
    /// Vehicle index of the car the penalty is applied to
    pub vehicle_idx: u8,
    /// Penalty given to the caa
    pub penalty_type: PenaltyType,
    /// Infringment done by the car
    pub infringement_type: InfringementType,
    /// Vehicle index of the other car involved
    pub other_vehicle_idx: u8,
    /// Time gained, or time spent doing action in seconds
    pub time: u8,
    /// Lap the penalty occurred on
    pub lap_num: u8,
    /// Number of places gained by this
    pub places_gained: u8,
}

/// Description of a speed trap event
#[derive(Debug, Copy, Clone, Default, PartialEq, Serialize)]
pub struct SpeedTrap {
    /// Vehicle index of the vehicle triggering speed trap
    pub vehicle_idx: u8,
    /// Top speed achieved in kilometres per hour
    pub speed: f32,
    /// Was this the overall fastest speed in the session (F1 2021+)
    pub is_overall_fastest_in_session: Option<bool>,
    /// Was this the car's fastest speed in the session (F1 2021+)
    pub is_personal_fastest_in_session: Option<bool>,
    /// Vehicle index of the vehicle that is the fastest in this session (F1 2022+)
    pub fastest_vehicle_idx_in_session: Option<u8>,
    /// Speed of the vehicle that is the fastest in this session
    pub fastest_speed_in_session: Option<f32>,
}

/// Description of a start lights event
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct StartLights {
    /// Number of lights showing
    pub number_of_lights: u8,
}

/// Description of a drive through penalty served event
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct DriveThroughPenaltyServed {
    /// Vehicle index of the vehicle serving drive through
    pub vehicle_idx: u8,
}

/// Description of a stop and go penalty served event
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct StopGoPenaltyServed {
    /// Vehicle index of the vehicle serving a stop and go
    pub vehicle_idx: u8,
}

/// Description of a flashback event
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub struct Flashback {
    /// Frame identifier flashed back to
    pub frame_identifier: u32,
    ///Session time flashed back to
    pub session_time: f32,
}

/// Description of a buttons event
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub struct Buttons {
    /// Bit flags specifying which buttons are being pressed currently
    pub button_status: u32,
}

/// List of possible events
///
/// The following packets were introduced in F1 2020:
/// * [`Event::Penalty`]
/// * [`Event::SpeedTrap`]
///
/// The following packets were introduced in F1 2021:
/// * [`Event::StartLights`]
/// * [`Event::LightsOut`]
/// * [`Event::DriveThroughPenaltyServed`]
/// * [`Event::StopGoPenaltyServed`]
/// * [`Event::Flashback`]
/// * [`Event::Buttons`]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
#[serde(tag = "event_type")]
pub enum Event {
    /// Sent when the session starts
    SessionStarted,
    /// Sent when the session ends
    SessionEnded,
    /// When a driver achieves the fastest lap
    FastestLap(FastestLap),
    /// When a driver retires
    Retirement(Retirement),
    /// Race control have enabled DRS
    DRSEnabled,
    /// Race control have disabled DRS
    DRSDisabled,
    /// Your team mate has entered the pits
    TeamMateInPits(TeamMateInPits),
    /// The chequered flag has been waved
    ChequeredFlag,
    /// The race winner is announced
    RaceWinner(RaceWinner),
    /// A penalty has been issued
    Penalty(Penalty),
    /// Speed trap has been triggered by fastest speed
    SpeedTrap(SpeedTrap),
    /// Start lights – number shown
    StartLights(StartLights),
    /// Lights out
    LightsOut,
    /// Drive through penalty served
    DriveThroughPenaltyServed(DriveThroughPenaltyServed),
    /// Stop go penalty served
    StopGoPenaltyServed(StopGoPenaltyServed),
    /// Flashback activated
    Flashback(Flashback),
    /// Button status changed
    Buttons(Buttons),
}

impl Event {
    /// Get a human readable description of an event
    pub fn description<'a>(self) -> &'a str {
        match self {
            Event::SessionStarted => "Session Started",
            Event::SessionEnded => "Session Ended",
            Event::FastestLap(_) => "Fastest Lap",
            Event::Retirement(_) => "Retirement",
            Event::DRSEnabled => "DRS Enabled",
            Event::DRSDisabled => "DRS Disabled",
            Event::TeamMateInPits(_) => "Teammate In Pits",
            Event::ChequeredFlag => "Chequered Flag",
            Event::RaceWinner(_) => "Race Winner",
            Event::Penalty(_) => "Penalty",
            Event::SpeedTrap(_) => "Speed Trap",
            Event::StartLights(_) => "Number of start lights shown",
            Event::LightsOut => "Lights out",
            Event::DriveThroughPenaltyServed(_) => "Drive through penalty served",
            Event::StopGoPenaltyServed(_) => "Stop and go penalty served",
            Event::Flashback(_) => "Flashback activated",
            Event::Buttons(_) => "Button status changed",
        }
    }

    /// Index of the vehicle involved in the event, if any.
    pub fn vehicle_idx(self) -> Option<u8> {
        match self {
            Event::FastestLap(e) => Some(e.vehicle_idx),
            Event::Retirement(e) => Some(e.vehicle_idx),
            Event::TeamMateInPits(e) => Some(e.vehicle_idx),
            Event::RaceWinner(e) => Some(e.vehicle_idx),
            Event::Penalty(e) => Some(e.vehicle_idx),
            Event::SpeedTrap(e) => Some(e.vehicle_idx),
            Event::DriveThroughPenaltyServed(e) => Some(e.vehicle_idx),
            Event::StopGoPenaltyServed(e) => Some(e.vehicle_idx),
            _ => None,
        }
    }
}

/// This packet gives details of events that happen during the course of a session.
///
/// Frequency: When the event occurs
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PacketEventData {
    /// Packet header
    pub header: PacketHeader,
    /// The event
    pub event: Event,
}
