use std::io::BufRead;

use serde::Deserialize;

use crate::packet::car_status::*;
use crate::packet::generic::{TyreCompound, TyreCompoundVisual, WheelData};
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;
use super::generic::unpack_flag;

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

fn unpack_tyre_compound(value: u8) -> Result<TyreCompound, UnpackError> {
    match value {
        16 => Ok(TyreCompound::C5),
        17 => Ok(TyreCompound::C4),
        18 => Ok(TyreCompound::C3),
        19 => Ok(TyreCompound::C2),
        20 => Ok(TyreCompound::C1),
        7 => Ok(TyreCompound::Inter),
        8 => Ok(TyreCompound::Wet),
        9 => Ok(TyreCompound::ClassicDry),
        10 => Ok(TyreCompound::ClassicWet),
        11 => Ok(TyreCompound::F2SuperSoft),
        12 => Ok(TyreCompound::F2Soft),
        13 => Ok(TyreCompound::F2Medium),
        14 => Ok(TyreCompound::F2Hard),
        15 => Ok(TyreCompound::F2Wet),
        0 | 255 => Ok(TyreCompound::Invalid),
        _ => Err(UnpackError(format!(
            "Invalid TyreCompound value: {}",
            value
        ))),
    }
}

fn unpack_tyre_compound_visual(value: u8) -> Result<TyreCompoundVisual, UnpackError> {
    match value {
        16 => Ok(TyreCompoundVisual::Soft),
        17 => Ok(TyreCompoundVisual::Medium),
        18 => Ok(TyreCompoundVisual::Hard),
        7 => Ok(TyreCompoundVisual::Inter),
        8 => Ok(TyreCompoundVisual::Wet),
        9 => Ok(TyreCompoundVisual::ClassicDry),
        10 => Ok(TyreCompoundVisual::ClassicWet),
        11 => Ok(TyreCompoundVisual::F2SuperSoft),
        12 => Ok(TyreCompoundVisual::F2Soft),
        13 => Ok(TyreCompoundVisual::F2Medium),
        14 => Ok(TyreCompoundVisual::F2Hard),
        15 => Ok(TyreCompoundVisual::F2Wet),
        0 => Ok(TyreCompoundVisual::Invalid),
        _ => Err(UnpackError(format!(
            "Invalid TyreCompoundVisual value: {}",
            value
        ))),
    }
}

fn unpack_ers_deploy_mode(value: u8) -> Result<ERSDeployMode, UnpackError> {
    match value {
        0 => Ok(ERSDeployMode::None),
        1 => Ok(ERSDeployMode::Low),
        2 => Ok(ERSDeployMode::Medium),
        3 => Ok(ERSDeployMode::High),
        4 => Ok(ERSDeployMode::Overtake),
        5 => Ok(ERSDeployMode::Hotlap),
        _ => Err(UnpackError(format!(
            "Invalid ERSDeployMode value: {}",
            value
        ))),
    }
}

/// This packet details car statuses for all the cars in the race. It includes values such as the damage readings on the car.
///
/// Frequency: Rate as specified in menus
/// Size: 1143 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// traction_control:            0 (off) - 2 (high)
/// anti_lock_brakes:            0 (off) - 1 (on)
/// fuel_mix:                    fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
/// front_brake_bias:            front brake bias (percentage)
/// pit_limiter_status:          pit limiter status - 0 = off, 1 = on
/// fuel_in_tank:                current fuel mass
/// fuel_capacity:               fuel capacity
/// fuel_remaining_laps:         fuel remaining in terms of laps (value on MFD)
/// max_rpm:                     cars max RPM, point of rev limiter
/// idle_rpm:                    cars idle RPM
/// max_gears:                   maximum number of gears
/// drs_allowed:                 0 = not allowed, 1 = allowed, -1 = unknown
/// tyres_wear:                  tyre wear percentage
/// actual_tyre_compound:        f1 modern - 16 = c5, 17 = c4, 18 = c3, 19 = c2, 20 = c1
///                              7 = inter, 8 = wet
///                              f1 classic - 9 = dry, 10 = wet
///                              f2 – 11 = super soft, 12 = soft, 13 = medium, 14 = hard
///                              15 = wet
/// tyre_visual_compound:        f1 visual (can be different from actual compound)
///                              16 = soft, 17 = medium, 18 = hard, 7 = inter, 8 = wet
///                              f1 classic – same as above
///                              f2 – same as above
/// tyres_damage:                tyre damage (percentage)
/// front_left_wing_damage:      front left wing damage (percentage)
/// front_right_wing_damage:     front right wing damage (percentage)
/// rear_wing_damage:            rear wing damage (percentage)
/// engine_damage:               engine damage (percentage)
/// gear_box_damage:             gear box damage (percentage)
/// vehicle_fia_flags:           -1 = invalid/unknown, 0 = none, 1 = green
///                              2 = blue, 3 = yellow, 4 = red
/// ers_store_energy:            ERS energy store in joules
/// ers_deploy_mode:             ERS deployment mode, 0 = none, 1 = low, 2 = medium
///                              3 = high, 4 = overtake, 5 = hotlap
/// ers_harvested_this_lap_mguk: ERS energy harvested this lap by MGU-k
/// ers_harvested_this_lap_mguh: ERS energy harvested this lap by MGU-h
/// ers_deployed_this_lap:       ERS energy deployed this lap
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
    tyres_wear: WheelData<u8>,
    actual_tyre_compound: u8,
    visual_tyre_compound: u8,
    tyres_damage: WheelData<u8>,
    front_left_wing_damage: u8,
    front_right_wing_damage: u8,
    rear_wing_damage: u8,
    engine_damage: u8,
    gear_box_damage: u8,
    vehicle_fia_flags: i8,
    ers_store_energy: f32,
    ers_deploy_mode: u8,
    ers_harvested_this_lap_mguk: f32,
    ers_harvested_this_lap_mguh: f32,
    ers_deployed_this_lap: f32,
}

impl CarStatusData {
    fn from_2019(packet: &RawCarStatus) -> Result<Self, UnpackError> {
        let traction_control = unpack_traction_control(packet.traction_control)?;
        let fuel_mix = unpack_fuel_mix(packet.fuel_mix)?;
        let drs_status = unpack_drs(packet.drs_allowed)?;
        let actual_tyre_compound = unpack_tyre_compound(packet.actual_tyre_compound)?;
        let visual_tyre_compound = unpack_tyre_compound_visual(packet.visual_tyre_compound)?;
        let vehicle_fia_flag = unpack_flag(packet.vehicle_fia_flags)?;
        let ers_deploy_mode = unpack_ers_deploy_mode(packet.ers_deploy_mode)?;

        Ok(CarStatusData::new(
            traction_control,
            packet.anti_lock_brakes,
            fuel_mix,
            packet.front_brake_bias,
            packet.pit_limiter,
            packet.fuel_in_tank,
            packet.fuel_capacity,
            packet.fuel_remaining_laps,
            packet.max_rpm,
            packet.idle_rpm,
            packet.max_gears,
            drs_status,
            None,
            packet.tyres_wear,
            actual_tyre_compound,
            visual_tyre_compound,
            None,
            packet.tyres_damage,
            packet.front_left_wing_damage,
            packet.front_right_wing_damage,
            packet.rear_wing_damage,
            false,
            packet.engine_damage,
            packet.gear_box_damage,
            vehicle_fia_flag,
            packet.ers_store_energy,
            ers_deploy_mode,
            packet.ers_harvested_this_lap_mguk,
            packet.ers_harvested_this_lap_mguh,
            packet.ers_deployed_this_lap,
        ))
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
        .map(CarStatusData::from_2019)
        .collect::<Result<Vec<CarStatusData>, UnpackError>>()?;

    Ok(PacketCarStatusData::new(header, car_status_data))
}
