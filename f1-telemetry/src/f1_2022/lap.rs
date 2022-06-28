use std::io::BufRead;

use serde::Deserialize;

use crate::packet::header::PacketHeader;
use crate::packet::lap::{DriverStatus, LapData, PacketLapData, PitStatus, Sector};
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

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

fn unpack_sector(value: u8) -> Result<Sector, UnpackError> {
    match value {
        0 => Ok(Sector::Sector1),
        1 => Ok(Sector::Sector2),
        2 => Ok(Sector::Sector3),
        _ => Err(UnpackError(format!("Invalid Sector value: {}", value))),
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

#[derive(Deserialize)]
struct RawPacketData {
    /// Lap data for all cars on track
    lap_data: [RawLapData; NUMBER_CARS],
    /// Index of Personal Best car in time trial (255 if invalid)
    time_trial_personal_best_car_idx: u8,
    /// Index of Rival car in time trial (255 if invalid)
    time_trial_rival_car_idx: u8,
}

impl PacketLapData {
    fn from_2022(header: PacketHeader, packet_data: RawPacketData) -> Result<Self, UnpackError> {
        let lap_data = packet_data
            .lap_data
            .iter()
            .map(LapData::from_2022)
            .collect::<Result<Vec<LapData>, UnpackError>>()?;

        let time_trial_personal_best_car_idx = match packet_data.time_trial_personal_best_car_idx {
            255 => None,
            idx => Some(idx),
        };
        let time_trial_rival_car_idx = match packet_data.time_trial_rival_car_idx {
            255 => None,
            idx => Some(idx),
        };

        Ok(Self {
            header,
            lap_data,
            time_trial_personal_best_car_idx,
            time_trial_rival_car_idx,
        })
    }
}

/// The lap data packet gives details of all the cars in the session.
///
/// Frequency: Rate as specified in menus
/// Size: 972 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// last_lap_time:                 Last lap time in milliseconds
/// current_lap_time:              Current time around the lap in milliseconds
/// sector_1_time:                 Sector 1 time in milliseconds
/// sector_2_time:                 Sector 2 time in milliseconds
/// lap_distance:                  Distance vehicle is around current lap in metres – could
///                                be negative if line hasn’t been crossed yet
/// total_distance:                Total distance travelled in session in metres – could
///                                be negative if line hasn’t been crossed yet
/// safety_car_delta:              Delta in seconds for safety car
/// car_position:                  Car race position
/// current_lap_num:               Current lap number
/// pit_status:                    Pitting status - 0 = none, 1 = pitting, 2 = in pit area
/// number_pit_stops:              Number of pit stops taken in this race
/// sector:                        0 = sector1, 1 = sector2, 2 = sector3
/// current_lap_invalid:           Current lap invalid - 0 = valid, 1 = invalid
/// penalties:                     Accumulated time penalties in seconds to be added
/// warnings:                      Accumulated number of warnings issued
/// number_unserved_drive_through: Number of drive through penalties left to serve
/// number_unserved_stop_go:       Number of stop and go penalties left to serve
/// grid_position:                 Grid position the vehicle started the race in
/// driver_status:                 Status of driver - 0 = in garage, 1 = flying lap
///                                2 = in lap, 3 = out lap, 4 = on track
/// result_status:                 Result status - 0 = invalid, 1 = inactive, 2 = active
///                                3 = finished, 4 = did not finish, 5 = disqualified
///                                6 = not classified, 7 = retired
/// pit_lane_timer_active:         Pit lane timing, 0 = inactive, 1 = active
/// pit_lane_time_in_lane:         If active, the current time spent in the pit lane in milliseconds
/// pit_stop_time:                 Time of the actual pit stop in milliseconds
/// pit_stop_should_serve_penalty: Whether the car should serve a penalty at this stop
/// ```
#[derive(Deserialize)]
struct RawLapData {
    last_lap_time: u32,
    current_lap_time: u32,
    sector_1_time: u16,
    sector_2_time: u16,
    lap_distance: f32,
    total_distance: f32,
    safety_car_delta: f32,
    car_position: u8,
    current_lap_num: u8,
    pit_status: u8,
    number_pit_stops: u8,
    sector: u8,
    current_lap_invalid: bool,
    penalties: u8,
    warnings: u8,
    number_unserved_drive_through: u8,
    number_unserved_stop_go: u8,
    grid_position: u8,
    driver_status: u8,
    result_status: u8,
    pit_lane_timer_active: bool,
    pit_lane_time_in_lane: u16,
    pit_stop_time: u16,
    pit_stop_should_serve_penalty: bool,
}

impl LapData {
    fn from_2022(car_lap_data: &RawLapData) -> Result<Self, UnpackError> {
        let pit_status = unpack_pit_status(car_lap_data.pit_status)?;
        let sector = unpack_sector(car_lap_data.sector)?;
        let driver_status = unpack_driver_status(car_lap_data.driver_status)?;
        let result_status = unpack_result_status(car_lap_data.result_status)?;

        Ok(Self {
            last_lap_time: car_lap_data.last_lap_time,
            current_lap_time: car_lap_data.current_lap_time,
            sector_1_time: car_lap_data.sector_1_time,
            sector_2_time: car_lap_data.sector_2_time,
            lap_distance: car_lap_data.lap_distance,
            total_distance: car_lap_data.total_distance,
            safety_car_delta: car_lap_data.safety_car_delta,
            car_position: car_lap_data.car_position,
            current_lap_num: car_lap_data.current_lap_num,
            pit_status,
            number_pit_stops: car_lap_data.number_pit_stops,
            sector,
            current_lap_invalid: car_lap_data.current_lap_invalid,
            penalties: car_lap_data.penalties,
            warnings: car_lap_data.warnings,
            number_unserved_drive_through: car_lap_data.number_unserved_drive_through,
            number_unserved_stop_go: car_lap_data.number_unserved_stop_go,
            grid_position: car_lap_data.grid_position,
            driver_status,
            result_status,
            pit_lane_timer_active: car_lap_data.pit_lane_timer_active,
            pit_lane_time_in_lane: car_lap_data.pit_lane_time_in_lane,
            pit_stop_time: car_lap_data.pit_stop_time,
            pit_stop_should_serve_penalty: car_lap_data.pit_stop_should_serve_penalty,
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

    let packet_data: RawPacketData = bincode::deserialize_from(reader)?;

    PacketLapData::from_2022(header, packet_data)
}
