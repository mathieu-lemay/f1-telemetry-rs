use crate::packet::generic::ResultStatus;

use super::header::PacketHeader;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PitStatus {
    None,
    Pitting,
    PitLane,
}

impl Default for PitStatus {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DriverStatus {
    Garage,
    FlyingLap,
    InLap,
    OutLap,
    OnTrack,
}

impl Default for DriverStatus {
    fn default() -> Self {
        Self::Garage
    }
}

/// This type is used for the `lap_data` array of the [`PacketLapData`] type.
///
/// ## Specification
/// ```text
/// last_lap_time:                 Last lap time in milliseconds
/// current_lap_time:              Current time around the lap in milliseconds
/// sector_1_time:                 Sector 1 time in milliseconds
/// sector_2_time:                 Sector 2 time in milliseconds
/// best_lap_time:                 Best lap time of the session in milliseconds
/// best_lap_num:                  Lap number best time achieved on
/// best_lap_sector_1_time:        Sector 1 time of best lap in the session in milliseconds
/// best_lap_sector_2_time:        Sector 2 time of best lap in the session in milliseconds
/// best_lap_sector_3_time:        Sector 3 time of best lap in the session in milliseconds
/// best_overall_sector_1_time:    Best overall sector 1 time of the session in milliseconds
/// best_overall_sector_1_lap_num: Lap number best overall sector 1 time achieved on
/// best_overall_sector_2_time:    Best overall sector 2 time of the session in milliseconds
/// best_overall_sector_2_lap_num: Lap number best overall sector 2 time achieved on
/// best_overall_sector_3_time:    Best overall sector 3 time of the session in milliseconds
/// best_overall_sector_3_lap_num: Lap number best overall sector 3 time achieved on
/// lap_distance:                  Distance vehicle is around current lap in metres – could
///                                be negative if line hasn’t been crossed yet
/// total_distance:                Total distance travelled in session in metres – could
///                                be negative if line hasn’t been crossed yet
/// safety_car_delta:              Delta in seconds for safety car
/// car_position:                  Car race position
/// current_lap_num:               Current lap number
/// pit_status:                    Pitting status. See [`PitStatus`].
/// number_pit_stops:              Number of pit stops taken in this race
/// sector:                        0 = sector1, 1 = sector2, 2 = sector3
/// current_lap_invalid:           Current lap invalid
/// penalties:                     Accumulated time penalties in seconds to be added
/// warnings:                      Accumulated number of warnings issued
/// number_unserved_drive_through: Number of drive through penalties left to serve
/// number_unserved_stop_go:       Number of stop and go penalties left to serve
/// grid_position:                 Grid position the vehicle started the race in
/// driver_status:                 Status of driver. See [`DriverStatus`].
/// result_status:                 Result status. See [`ResultStatus`].
/// pit_lane_timer_active:         Pit lane timing, 0 = inactive, 1 = active
/// pit_lane_time_in_lane:         If active, the current time spent in the pit lane in milliseconds
/// pit_stop_time:                 Time of the actual pit stop in milliseconds
/// pit_stop_should_serve_penalty: Whether the car should serve a penalty at this stop
/// ```
/// See also: [`DriverStatus`], [`PitStatus`], [`ResultStatus`]
#[derive(Debug, PartialEq, Default)]
pub struct LapData {
    pub last_lap_time: u32,
    pub current_lap_time: u32,
    pub sector_1_time: u16,
    pub sector_2_time: u16,
    pub best_lap_time: u32,
    pub best_lap_num: u8,
    pub best_lap_sector_1_time: u16,
    pub best_lap_sector_2_time: u16,
    pub best_lap_sector_3_time: u16,
    pub best_overall_sector_1_time: u16,
    pub best_overall_sector_1_lap_num: u8,
    pub best_overall_sector_2_time: u16,
    pub best_overall_sector_2_lap_num: u8,
    pub best_overall_sector_3_time: u16,
    pub best_overall_sector_3_lap_num: u8,
    pub lap_distance: f32,
    pub total_distance: f32,
    pub safety_car_delta: f32,
    pub car_position: u8,
    pub current_lap_num: u8,
    pub pit_status: PitStatus,
    pub number_pit_stops: u8,
    pub sector: u8,
    pub current_lap_invalid: bool,
    pub penalties: u8,
    pub warnings: u8,
    pub number_unserved_drive_through: u8,
    pub number_unserved_stop_go: u8,
    pub grid_position: u8,
    pub driver_status: DriverStatus,
    pub result_status: ResultStatus,
    pub pit_lane_timer_active: bool,
    pub pit_lane_time_in_lane: u16,
    pub pit_stop_time: u16,
    pub pit_stop_should_serve_penalty: bool,
}

/// The lap data packet gives details of all the cars in the session.
///
/// Frequency: Rate as specified in menus
///
/// ## Specification
/// ```text
/// header:   Header
/// lap_data: Lap data for all cars on track
/// ```
#[derive(Debug, PartialEq)]
pub struct PacketLapData {
    pub header: PacketHeader,
    pub lap_data: Vec<LapData>,
}
