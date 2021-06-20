use getset::{CopyGetters, Getters};

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
/// last_lap_time:                 Last lap time in seconds
/// current_lap_time:              Current time around the lap in seconds
/// sector_1_time:                 Sector 1 time in milliseconds
/// sector_2_time:                 Sector 2 time in milliseconds
/// best_lap_time:                 Best lap time of the session in seconds
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
/// sector:                        0 = sector1, 1 = sector2, 2 = sector3
/// current_lap_invalid:           Current lap invalid - 0 = valid, 1 = invalid
/// penalties:                     Accumulated time penalties in seconds to be added
/// grid_position:                 Grid position the vehicle started the race in
/// driver_status:                 Status of driver. See [`DriverStatus`].
/// result_status:                 Result status. See [`ResultStatus`].
/// ```
/// [`PacketLapData`]: ./struct.PacketLapData.html
#[derive(Debug, Default, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct LapData {
    last_lap_time: u32,
    current_lap_time: u32,
    sector_1_time: u16,
    sector_2_time: u16,
    best_lap_time: u32,
    best_lap_num: u8,
    best_lap_sector_1_time: u16,
    best_lap_sector_2_time: u16,
    best_lap_sector_3_time: u16,
    best_overall_sector_1_time: u16,
    best_overall_sector_1_lap_num: u8,
    best_overall_sector_2_time: u16,
    best_overall_sector_2_lap_num: u8,
    best_overall_sector_3_time: u16,
    best_overall_sector_3_lap_num: u8,
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
    pub(crate) fn from_2019(
        last_lap_time: u32,
        current_lap_time: u32,
        best_lap_time: u32,
        sector_1_time: u32,
        sector_2_time: u32,
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
        let sector_1_time = sector_1_time as u16;
        let sector_2_time = sector_2_time as u16;

        LapData {
            last_lap_time,
            current_lap_time,
            sector_1_time,
            sector_2_time,
            best_lap_time,
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
            ..Default::default()
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_2020(
        last_lap_time: u32,
        current_lap_time: u32,
        sector_1_time: u16,
        sector_2_time: u16,
        best_lap_time: u32,
        best_lap_num: u8,
        best_lap_sector_1_time: u16,
        best_lap_sector_2_time: u16,
        best_lap_sector_3_time: u16,
        best_overall_sector_1_time: u16,
        best_overall_sector_1_lap_num: u8,
        best_overall_sector_2_time: u16,
        best_overall_sector_2_lap_num: u8,
        best_overall_sector_3_time: u16,
        best_overall_sector_3_lap_num: u8,
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
            sector_1_time,
            sector_2_time,
            best_lap_time,
            best_lap_num,
            best_lap_sector_1_time,
            best_lap_sector_2_time,
            best_lap_sector_3_time,
            best_overall_sector_1_time,
            best_overall_sector_1_lap_num,
            best_overall_sector_2_time,
            best_overall_sector_2_lap_num,
            best_overall_sector_3_time,
            best_overall_sector_3_lap_num,
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
