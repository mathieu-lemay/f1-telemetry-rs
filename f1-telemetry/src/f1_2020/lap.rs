use std::io::BufRead;

use serde::Deserialize;

use crate::packet::header::PacketHeader;
use crate::packet::lap::{DriverStatus, LapData, PacketLapData, PitStatus};
use crate::packet::UnpackError;
use crate::utils::{assert_packet_size, seconds_to_millis};

use super::consts::*;
use super::generic::unpack_result_status;

fn unpack_pit_status(value: u8) -> Result<PitStatus, UnpackError> {
    match value {
        0 => Ok(PitStatus::None),
        1 => Ok(PitStatus::Pitting),
        2 => Ok(PitStatus::PitLane),
        _ => Err(UnpackError(format!("Invalid PitStatus value: {}", value))),
    }
}

fn unpack_driver_status(value: u8) -> Result<DriverStatus, UnpackError> {
    match value {
        0 => Ok(DriverStatus::Garage),
        1 => Ok(DriverStatus::FlyingLap),
        2 => Ok(DriverStatus::InLap),
        3 => Ok(DriverStatus::OutLap),
        4 => Ok(DriverStatus::OnTrack),
        _ => Err(UnpackError(format!(
            "Invalid DriverStatus value: {}",
            value
        ))),
    }
}

/// The lap data packet gives details of all the cars in the session.
///
/// Frequency: Rate as specified in menus
/// Size: 1190 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// last_lap_time:                  Last lap time in seconds
/// current_lap_time:               Current time around the lap in seconds
/// sector_1_time:                  Sector 1 time in milliseconds
/// sector_2_time:                  Sector 2 time in milliseconds
/// best_lap_time:                  Best lap time of the session in seconds
/// best_lap_num:                   Lap number best time achieved on
/// best_lap_sector_1_time:         Sector 1 time of best lap in the session in milliseconds
/// best_lap_sector_2_time:         Sector 2 time of best lap in the session in milliseconds
/// best_lap_sector_3_time:         Sector 3 time of best lap in the session in milliseconds
/// best_overall_sector_1_time:     Best overall sector 1 time of the session in milliseconds
/// best_overall_sector_1_lap_num:  Lap number best overall sector 1 time achieved on
/// best_overall_sector_2_time:     Best overall sector 2 time of the session in milliseconds
/// best_overall_sector_2_lap_num:  Lap number best overall sector 2 time achieved on
/// best_overall_sector_3_time:     Best overall sector 3 time of the session in milliseconds
/// best_overall_sector_3_lap_num:  Lap number best overall sector 3 time achieved on
/// lap_distance:                   Distance vehicle is around current lap in metres – could
///                                 be negative if line hasn’t been crossed yet
/// total_distance:                 Total distance travelled in session in metres – could
///                                 be negative if line hasn’t been crossed yet
///
/// safety_car_delta:               Delta in seconds for safety car
/// car_position:                   Car race position
/// current_lap_num:                Current lap number
/// pit_status:                     Pitting status - 0 = none, 1 = pitting, 2 = in pit area
/// sector:                         0 = sector1, 1 = sector2, 2 = sector3
/// current_lap_invalid:            Current lap invalid - 0 = valid, 1 = invalid
/// penalties:                      Accumulated time penalties in seconds to be added
/// grid_position:                  Grid position the vehicle started the race in
/// driver_status:                  Status of driver - 0 = in garage, 1 = flying lap
///                                 2 = in lap, 3 = out lap, 4 = on track
/// result_status:                  Result status - 0 = invalid, 1 = inactive, 2 = active
///                                 3 = finished, 4 = disqualified, 5 = not classified
///                                 6 = retired
/// ```
#[derive(Deserialize)]
struct RawLapData {
    last_lap_time: f32,
    current_lap_time: f32,
    sector_1_time: u16,
    sector_2_time: u16,
    best_lap_time: f32,
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
    pit_status: u8,
    sector: u8,
    current_lap_invalid: bool,
    penalties: u8,
    grid_position: u8,
    driver_status: u8,
    result_status: u8,
}

impl LapData {
    fn from_2020(car_lap_data: &RawLapData) -> Result<Self, UnpackError> {
        let last_lap_time = seconds_to_millis(car_lap_data.last_lap_time as f64);
        let current_lap_time = seconds_to_millis(car_lap_data.current_lap_time as f64);
        let best_lap_time = seconds_to_millis(car_lap_data.best_lap_time as f64);
        let pit_status = unpack_pit_status(car_lap_data.pit_status)?;
        let driver_status = unpack_driver_status(car_lap_data.driver_status)?;
        let result_status = unpack_result_status(car_lap_data.result_status)?;

        Ok(Self {
            last_lap_time,
            current_lap_time,
            sector_1_time: car_lap_data.sector_1_time,
            sector_2_time: car_lap_data.sector_2_time,
            best_lap_time,
            best_lap_num: car_lap_data.best_lap_num,
            best_lap_sector_1_time: car_lap_data.best_lap_sector_1_time,
            best_lap_sector_2_time: car_lap_data.best_lap_sector_2_time,
            best_lap_sector_3_time: car_lap_data.best_lap_sector_3_time,
            best_overall_sector_1_time: car_lap_data.best_overall_sector_1_time,
            best_overall_sector_1_lap_num: car_lap_data.best_overall_sector_1_lap_num,
            best_overall_sector_2_time: car_lap_data.best_overall_sector_2_time,
            best_overall_sector_2_lap_num: car_lap_data.best_overall_sector_2_lap_num,
            best_overall_sector_3_time: car_lap_data.best_overall_sector_3_time,
            best_overall_sector_3_lap_num: car_lap_data.best_overall_sector_3_lap_num,
            lap_distance: car_lap_data.lap_distance,
            total_distance: car_lap_data.total_distance,
            safety_car_delta: car_lap_data.safety_car_delta,
            car_position: car_lap_data.car_position,
            current_lap_num: car_lap_data.current_lap_num,
            pit_status,
            sector: car_lap_data.sector,
            current_lap_invalid: car_lap_data.current_lap_invalid,
            penalties: car_lap_data.penalties,
            grid_position: car_lap_data.grid_position,
            driver_status,
            result_status,
            ..Default::default()
        })
    }
}

pub(crate) fn parse_lap_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketLapData, UnpackError> {
    assert_packet_size(size, LAP_DATA_PACKET_SIZE)?;

    let lap_data: [RawLapData; NUMBER_CARS] = bincode::deserialize_from(reader)?;

    let lap_data = lap_data
        .iter()
        .map(LapData::from_2020)
        .collect::<Result<Vec<LapData>, UnpackError>>()?;

    Ok(PacketLapData { header, lap_data })
}
