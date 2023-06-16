use serde::Serialize;

use crate::packet::generic::ResultStatus;

use super::header::PacketHeader;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum PitStatus {
    #[default]
    None,
    Pitting,
    PitLane,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Serialize)]
pub enum DriverStatus {
    #[default]
    Garage,
    FlyingLap,
    InLap,
    OutLap,
    OnTrack,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize)]
pub enum Sector {
    #[default]
    Sector1,
    Sector2,
    Sector3,
}

/// Lap data for a car on track
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct LapData {
    /// Last lap time in milliseconds
    pub last_lap_time: u32,
    /// Current time around the lap in milliseconds
    pub current_lap_time: u32,
    /// Sector 1 time in milliseconds
    pub sector_1_time: u16,
    /// Sector 1 whole minute part. New in F1 23.
    pub sector_1_time_minutes: u8,
    /// Sector 2 time in milliseconds
    pub sector_2_time: u16,
    /// Sector 2 whole minute part. New in F1 23.
    pub sector_2_time_minutes: u8,
    /// Time delta to car in front in milliseconds
    pub delta_to_car_in_front: u16,
    /// Time delta to race leader in milliseconds
    pub delta_to_race_leader: u16,
    /// Best lap time of the session in milliseconds
    pub best_lap_time: u32,
    /// Lap number best time achieved on
    pub best_lap_num: u8,
    /// Sector 1 time of best lap in the session in milliseconds
    pub best_lap_sector_1_time: u16,
    /// Sector 2 time of best lap in the session in milliseconds
    pub best_lap_sector_2_time: u16,
    /// Sector 3 time of best lap in the session in milliseconds
    pub best_lap_sector_3_time: u16,
    /// Best overall sector 1 time of the session in milliseconds
    pub best_overall_sector_1_time: u16,
    /// Lap number best overall sector 1 time achieved on
    pub best_overall_sector_1_lap_num: u8,
    /// Best overall sector 2 time of the session in milliseconds
    pub best_overall_sector_2_time: u16,
    /// Lap number best overall sector 2 time achieved on
    pub best_overall_sector_2_lap_num: u8,
    /// Best overall sector 3 time of the session in milliseconds
    pub best_overall_sector_3_time: u16,
    /// Lap number best overall sector 3 time achieved on
    pub best_overall_sector_3_lap_num: u8,
    /// Distance vehicle is around current lap in metres – could be negative if line hasn’t been crossed yet
    pub lap_distance: f32,
    /// Total distance travelled in session in metres – could be negative if line hasn’t been crossed yet
    pub total_distance: f32,
    /// Delta in seconds for safety car
    pub safety_car_delta: f32,
    /// Car race position
    pub car_position: u8,
    /// Current lap number
    pub current_lap_num: u8,
    /// Pitting status
    pub pit_status: PitStatus,
    /// Number of pit stops taken in this race
    pub number_pit_stops: u8,
    /// Current sector
    pub sector: Sector,
    /// Current lap invalid
    pub current_lap_invalid: bool,
    /// Accumulated time penalties in seconds to be added
    pub penalties: u8,
    /// Accumulated number of warnings issued
    pub total_warnings: u8,
    /// Accumulated number of corner cutting warnings issued
    pub corner_cutting_warnings: u8,
    /// Number of drive through penalties left to serve
    pub number_unserved_drive_through: u8,
    /// Number of stop and go penalties left to serve
    pub number_unserved_stop_go: u8,
    /// Grid position the vehicle started the race in
    pub grid_position: u8,
    /// Status of driver
    pub driver_status: DriverStatus,
    /// Result status
    pub result_status: ResultStatus,
    /// Is the pit lane timer active or not
    pub pit_lane_timer_active: bool,
    /// If active, the current time spent in the pit lane in milliseconds
    pub pit_lane_time_in_lane: u16,
    /// Time of the actual pit stop in milliseconds
    pub pit_stop_time: u16,
    /// Whether the car should serve a penalty at this stop
    pub pit_stop_should_serve_penalty: bool,
}

/// The lap data packet gives details of all the cars in the session.
///
/// Frequency: Rate as specified in menus
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PacketLapData {
    /// Packet header
    pub header: PacketHeader,
    /// Lap data for all cars on track
    pub lap_data: Vec<LapData>,
    /// Index of Personal Best car in time trial (if available)
    pub time_trial_personal_best_car_idx: Option<u8>,
    /// Index of Rival car in time trial (if available)
    pub time_trial_rival_car_idx: Option<u8>,
}
