use super::header::PacketHeader;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FastestLap {
    pub vehicle_idx: u8,
    pub lap_time: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Retirement {
    pub vehicle_idx: u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TeamMateInPits {
    pub vehicle_idx: u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RaceWinner {
    pub vehicle_idx: u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
}

/// Penalty Event
///
/// ## Specification
/// ```text
/// vehicle_idx:       Vehicle index of the car the penalty is applied to
/// penalty_type:      Penalty type. See [`PenaltyType`].
/// infringement_type: Infringement type. See [`InfringementType`].
/// other_vehicle_idx: Vehicle index of the other car involved
/// time:              Time gained, or time spent doing action in seconds
/// lap_num:           Lap the penalty occurred on
/// places_gained:     Number of places gained by this
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Penalty {
    pub vehicle_idx: u8,
    pub penalty_type: PenaltyType,
    pub infringement_type: InfringementType,
    pub other_vehicle_idx: u8,
    pub time: u8,
    pub lap_num: u8,
    pub places_gained: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct SpeedTrap {
    pub vehicle_idx: u8,
    pub speed: f32,
    pub overall_fastest_in_session: Option<bool>,
    pub personal_fastest_in_session: Option<bool>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StartLights {
    pub number_of_lights: u8,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DriveThroughPenaltyServed {
    pub vehicle_idx: u8,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StopGoPenaltyServed {
    pub vehicle_idx: u8,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Flashback {
    pub frame_identifier: u32,
    pub session_time: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Buttons {
    pub button_status: u32,
}

/// List of possible events.
///
/// ## Specification
/// ```text
/// SessionStarted:            Sent when the session starts
/// SessionEnded:              Sent when the session ends
/// FastestLap:                When a driver achieves the fastest lap
/// Retirement:                When a driver retires
/// DRSEnabled:                Race control have enabled DRS
/// DRSDisabled:               Race control have disabled DRS
/// TeamMateInPits:            Your team mate has entered the pits
/// ChequeredFlag:             The chequered flag has been waved
/// RaceWinner:                The race winner is announced
///
/// Introduced in F1 2020
/// Penalty:                   A penalty has been issued
/// SpeedTrap:                 Speed trap has been triggered by fastest speed
///
/// Introduced in F1 2021
/// StartLights:               Start lights – number shown
/// LightsOut:                 Lights out
/// DriveThroughPenaltyServed: Drive through penalty served
/// StopGoPenaltyServed:       Stop go penalty served
/// Flashback:                 Flashback activated
/// Buttons:                   Button status changed
/// ```
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Event {
    SessionStarted,
    SessionEnded,
    FastestLap(FastestLap),
    Retirement(Retirement),
    DRSEnabled,
    DRSDisabled,
    TeamMateInPits(TeamMateInPits),
    ChequeredFlag,
    RaceWinner(RaceWinner),
    Penalty(Penalty),
    SpeedTrap(SpeedTrap),
    StartLights(StartLights),
    LightsOut,
    DriveThroughPenaltyServed(DriveThroughPenaltyServed),
    StopGoPenaltyServed(StopGoPenaltyServed),
    Flashback(Flashback),
    Buttons(Buttons),
}

impl Event {
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
///
/// ## Specification
/// ```text
/// header: Header
/// event:  See [`Event`]
/// ```
///
/// See also [`Event`]
#[derive(Debug, PartialEq)]
pub struct PacketEventData {
    pub header: PacketHeader,
    pub event: Event,
}
