use getset::{CopyGetters, Getters};

use super::header::PacketHeader;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PitStatus {
    None,
    Pitting,
    PitLane,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DriverStatus {
    Garage,
    FlyingLap,
    InLap,
    OutLap,
    OnTrack,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ResultStatus {
    Invalid,
    Inactive,
    Active,
    Finished,
    Disqualified,
    NotClassified,
    Retired,
}

impl Default for ResultStatus {
    fn default() -> Self {
        ResultStatus::Invalid
    }
}

/// This type is used for the 20-element `lap_data` array of the [`PacketLapData`] type.
///
/// Size: 41 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// last_lap_time:       Last lap time in seconds
/// current_lap_time:    Current time around the lap in seconds
/// best_lap_time:       Best lap time of the session in seconds
/// sector_1_time:       Sector 1 time in seconds
/// sector_2_time:       Sector 2 time in seconds
/// lap_distance:        Distance vehicle is around current lap in metres – could
///                      be negative if line hasn’t been crossed yet
/// total_distance:      Total distance travelled in session in metres – could
///                      be negative if line hasn’t been crossed yet
/// safety_car_delta:    Delta in seconds for safety car
/// car_position:        Car race position
/// current_lap_num:     Current lap number
/// pit_status:          0 = none, 1 = pitting, 2 = in pit area
/// sector:              0 = sector1, 1 = sector2, 2 = sector3
/// current_lap_invalid: Current lap invalid - 0 = valid, 1 = invalid
/// penalties:           Accumulated time penalties in seconds to be added
/// grid_position:       Grid position the vehicle started the race in
/// driver_status:       Status of driver - 0 = in garage, 1 = flying lap
///                      2 = in lap, 3 = out lap, 4 = on track
/// result_status:       Result status - 0 = invalid, 1 = inactive, 2 = active
///                      3 = finished, 4 = disqualified, 5 = not classified
///                      6 = retired
/// ```
/// [`PacketLapData`]: ./struct.PacketLapData.html
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct LapData {
    last_lap_time: f32,
    current_lap_time: f32,
    best_lap_time: f32,
    sector_1_time: f32,
    sector_2_time: f32,
    lap_distance: f32,
    total_distance: f32,
    safety_car_delta: f32,
    car_position: u8,
    current_lap_num: u8,
    pit_status: PitStatus,
    sector: u8,
    current_lap_invalid: bool,
    penalties: u8,
    grid_position: u8,
    driver_status: DriverStatus,
    result_status: ResultStatus,
}

impl LapData {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        last_lap_time: f32,
        current_lap_time: f32,
        best_lap_time: f32,
        sector_1_time: f32,
        sector_2_time: f32,
        lap_distance: f32,
        total_distance: f32,
        safety_car_delta: f32,
        car_position: u8,
        current_lap_num: u8,
        pit_status: PitStatus,
        sector: u8,
        current_lap_invalid: bool,
        penalties: u8,
        grid_position: u8,
        driver_status: DriverStatus,
        result_status: ResultStatus,
    ) -> LapData {
        LapData {
            last_lap_time,
            current_lap_time,
            best_lap_time,
            sector_1_time,
            sector_2_time,
            lap_distance,
            total_distance,
            safety_car_delta,
            car_position,
            current_lap_num,
            pit_status,
            sector,
            current_lap_invalid,
            penalties,
            grid_position,
            driver_status,
            result_status,
        }
    }
}

/// The lap data packet gives details of all the cars in the session.
///
/// Frequency: Rate as specified in menus
///
/// Size: 843 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// header:   Header
/// lap_data: Lap data for all cars on track
/// ```
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct PacketLapData {
    header: PacketHeader,
    lap_data: Vec<LapData>,
}

impl PacketLapData {
    pub(crate) fn new(header: PacketHeader, lap_data: Vec<LapData>) -> PacketLapData {
        PacketLapData { header, lap_data }
    }
}
