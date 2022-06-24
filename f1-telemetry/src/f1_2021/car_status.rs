use std::io::BufRead;

use serde::Deserialize;

use crate::f1_2021::generic::{unpack_flag, unpack_tyre_compound, unpack_tyre_compound_visual};
use crate::packet::car_status::*;
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

fn unpack_traction_control(value: u8) -> Result<TractionControl, UnpackError> {
    match value {
        0 => Ok(TractionControl::Off),
        1 => Ok(TractionControl::Low),
        2 => Ok(TractionControl::High),
        _ => Err(UnpackError(format!(
            "Invalid TractionControl value: {}",
            value
        ))),
    }
}

fn unpack_fuel_mix(value: u8) -> Result<FuelMix, UnpackError> {
    match value {
        0 => Ok(FuelMix::Lean),
        1 => Ok(FuelMix::Standard),
        2 => Ok(FuelMix::Rich),
        3 => Ok(FuelMix::Max),
        _ => Err(UnpackError(format!("Invalid FuelMix value: {}", value))),
    }
}

fn unpack_drs(value: i8) -> Result<DRS, UnpackError> {
    match value {
        0 => Ok(DRS::NotAllowed),
        1 => Ok(DRS::Allowed),
        -1 => Ok(DRS::Unknown),
        _ => Err(UnpackError(format!("Invalid DRS value: {}", value))),
    }
}

fn unpack_ers_deploy_mode(value: u8) -> Result<ERSDeployMode, UnpackError> {
    match value {
        0 => Ok(ERSDeployMode::None),
        1 => Ok(ERSDeployMode::Medium),
        2 => Ok(ERSDeployMode::Hotlap),
        3 => Ok(ERSDeployMode::Overtake),
        _ => Err(UnpackError(format!(
            "Invalid ERSDeployMode value: {}",
            value
        ))),
    }
}

/// This packet details car statuses for all the cars in the race. It includes values such as the damage readings on the car.
///
/// Frequency: Rate as specified in menus
/// Size: 1058 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// traction_control:            Traction control - 0 = off, 1 = medium, 2 = full
/// anti_lock_brakes:            0 (off) - 1 (on)
/// fuel_mix:                    Fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
/// front_brake_bias:            Front brake bias (percentage)
/// pit_limiter_status:          Pit limiter status - 0 = off, 1 = on
/// fuel_in_tank:                Current fuel mass
/// fuel_capacity:               Fuel capacity
/// fuel_remaining_laps:         Fuel remaining in terms of laps (value on MFD)
/// max_rpm:                     Car's max RPM, point of rev limiter
/// idle_rpm:                    Car's idle RPM
/// max_gears:                   Maximum number of gears
/// drs_allowed:                 0 = not allowed, 1 = allowed
/// drs_activation_distance:     0 = DRS not available, non-zero - DRS will be available
///                              in [X] metres
/// actual_tyre_compound:        F1 modern - 16 = c5, 17 = c4, 18 = c3, 19 = c2, 20 = c1
///                              7 = inter, 8 = wet
///                              F1 classic - 9 = dry, 10 = wet
///                              F2 – 11 = super soft, 12 = soft, 13 = medium, 14 = hard
///                              15 = wet
/// tyre_visual_compound:        F1 visual (can be different from actual compound)
///                              16 = soft, 17 = medium, 18 = hard, 7 = inter, 8 = wet
///                              F1 classic – same as above
///                              F2 ‘19, 15 = wet, 19 – super soft, 20 = soft
///                              21 = medium , 22 = hard
/// tyres_age_laps               Age in laps of the current set of tyres
/// vehicle_fia_flags:           -1 = invalid/unknown, 0 = none, 1 = green
///                              2 = blue, 3 = yellow, 4 = red
/// ers_store_energy:            ERS energy store in joules
/// ers_deploy_mode:             ERS deployment mode, 0 = none, 1 = medium
///                              2 = hotlap, 3 = overtake
/// ers_harvested_this_lap_mguk: ERS energy harvested this lap by MGU-k
/// ers_harvested_this_lap_mguh: ERS energy harvested this lap by MGU-h
/// ers_deployed_this_lap:       ERS energy deployed this lap
/// network_paused:              Wether the car is paused in a network game
/// ```
#[derive(Deserialize)]
struct RawCarStatus {
    traction_control: u8,
    anti_lock_brakes: bool,
    fuel_mix: u8,
    front_brake_bias: u8,
    pit_limiter: bool,
    fuel_in_tank: f32,
    fuel_capacity: f32,
    fuel_remaining_laps: f32,
    max_rpm: u16,
    idle_rpm: u16,
    max_gears: u8,
    drs_allowed: i8,
    drs_activation_distance: u16,
    actual_tyre_compound: u8,
    visual_tyre_compound: u8,
    tyres_age_laps: u8,
    vehicle_fia_flags: i8,
    ers_store_energy: f32,
    ers_deploy_mode: u8,
    ers_harvested_this_lap_mguk: f32,
    ers_harvested_this_lap_mguh: f32,
    ers_deployed_this_lap: f32,
    network_paused: bool,
}

impl CarStatusData {
    fn from_2021(packet: &RawCarStatus) -> Result<Self, UnpackError> {
        let traction_control = unpack_traction_control(packet.traction_control)?;
        let fuel_mix = unpack_fuel_mix(packet.fuel_mix)?;
        let drs_status = unpack_drs(packet.drs_allowed)?;
        let actual_tyre_compound = unpack_tyre_compound(packet.actual_tyre_compound)?;
        let visual_tyre_compound = unpack_tyre_compound_visual(packet.visual_tyre_compound)?;
        let vehicle_fia_flag = unpack_flag(packet.vehicle_fia_flags)?;
        let ers_deploy_mode = unpack_ers_deploy_mode(packet.ers_deploy_mode)?;

        Ok(CarStatusData {
            traction_control,
            anti_lock_brakes: packet.anti_lock_brakes,
            fuel_mix,
            front_brake_bias: packet.front_brake_bias,
            pit_limiter: packet.pit_limiter,
            fuel_in_tank: packet.fuel_in_tank,
            fuel_capacity: packet.fuel_capacity,
            fuel_remaining_laps: packet.fuel_remaining_laps,
            max_rpm: packet.max_rpm,
            idle_rpm: packet.idle_rpm,
            max_gears: packet.max_gears,
            drs_status,
            drs_activation_distance: Some(packet.drs_activation_distance),
            actual_tyre_compound,
            visual_tyre_compound,
            tyre_age_laps: Some(packet.tyres_age_laps),
            vehicle_fia_flag,
            ers_store_energy: packet.ers_store_energy,
            ers_deploy_mode,
            ers_harvested_this_lap_mguk: packet.ers_harvested_this_lap_mguk,
            ers_harvested_this_lap_mguh: packet.ers_harvested_this_lap_mguh,
            ers_deployed_this_lap: packet.ers_deployed_this_lap,
            network_paused: packet.network_paused,
            ..Default::default()
        })
    }
}

pub fn parse_car_status_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketCarStatusData, UnpackError> {
    assert_packet_size(size, CAR_STATUS_PACKET_SIZE)?;

    let car_status: [RawCarStatus; NUMBER_CARS] = bincode::deserialize_from(reader)?;

    let car_status_data = car_status
        .iter()
        .map(CarStatusData::from_2021)
        .collect::<Result<Vec<CarStatusData>, UnpackError>>()?;

    Ok(PacketCarStatusData {
        header,
        car_status_data,
    })
}
