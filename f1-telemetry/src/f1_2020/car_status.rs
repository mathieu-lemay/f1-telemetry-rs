use std::io::BufRead;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::f1_2020::generic::{unpack_flag, unpack_tyre_compound, unpack_tyre_compound_visual};
use crate::packet::car_status::*;
use crate::packet::generic::WheelData;
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
        2 => Ok(ERSDeployMode::Overtake),
        3 => Ok(ERSDeployMode::Hotlap),
        _ => Err(UnpackError(format!(
            "Invalid ERSDeployMode value: {}",
            value
        ))),
    }
}

fn parse_car<T: BufRead>(reader: &mut T) -> Result<CarStatusData, UnpackError> {
    let traction_control = unpack_traction_control(reader.read_u8().unwrap())?;
    let anti_lock_brakes = reader.read_u8().unwrap() == 1;
    let fuel_mix = unpack_fuel_mix(reader.read_u8().unwrap())?;
    let front_brake_bias = reader.read_u8().unwrap();
    let pit_limiter = reader.read_u8().unwrap() == 1;
    let fuel_in_tank = reader.read_f32::<LittleEndian>().unwrap();
    let fuel_capacity = reader.read_f32::<LittleEndian>().unwrap();
    let fuel_remaining_laps = reader.read_f32::<LittleEndian>().unwrap();
    let max_rpm = reader.read_u16::<LittleEndian>().unwrap();
    let idle_rpm = reader.read_u16::<LittleEndian>().unwrap();
    let max_gears = reader.read_u8().unwrap();
    let drs_allowed = unpack_drs(reader.read_i8().unwrap())?;
    let drs_activation_distance = reader.read_u16::<LittleEndian>().unwrap();
    let tyres_wear = WheelData::new(
        reader.read_u8().unwrap(),
        reader.read_u8().unwrap(),
        reader.read_u8().unwrap(),
        reader.read_u8().unwrap(),
    );
    let actual_tyre_compound = unpack_tyre_compound(reader.read_u8().unwrap())?;
    let visual_tyre_compound = unpack_tyre_compound_visual(reader.read_u8().unwrap())?;
    let tyres_age_laps = reader.read_u8().unwrap();
    let tyres_damage = WheelData::new(
        reader.read_u8().unwrap(),
        reader.read_u8().unwrap(),
        reader.read_u8().unwrap(),
        reader.read_u8().unwrap(),
    );
    let front_left_wing_damage = reader.read_u8().unwrap();
    let front_right_wing_damage = reader.read_u8().unwrap();
    let rear_wing_damage = reader.read_u8().unwrap();
    let drs_fault = reader.read_u8().unwrap() == 1;
    let engine_damage = reader.read_u8().unwrap();
    let gear_box_damage = reader.read_u8().unwrap();
    let vehicle_fia_flags = unpack_flag(reader.read_i8().unwrap())?;
    let ers_store_energy = reader.read_f32::<LittleEndian>().unwrap();
    let ers_deploy_mode = unpack_ers_deploy_mode(reader.read_u8().unwrap())?;
    let ers_harvested_this_lap_mguk = reader.read_f32::<LittleEndian>().unwrap();
    let ers_harvested_this_lap_mguh = reader.read_f32::<LittleEndian>().unwrap();
    let ers_deployed_this_lap = reader.read_f32::<LittleEndian>().unwrap();

    Ok(CarStatusData::from_2020(
        traction_control,
        anti_lock_brakes,
        fuel_mix,
        front_brake_bias,
        pit_limiter,
        fuel_in_tank,
        fuel_capacity,
        fuel_remaining_laps,
        max_rpm,
        idle_rpm,
        max_gears,
        drs_allowed,
        drs_activation_distance,
        tyres_wear,
        actual_tyre_compound,
        visual_tyre_compound,
        tyres_age_laps,
        tyres_damage,
        front_left_wing_damage,
        front_right_wing_damage,
        rear_wing_damage,
        drs_fault,
        engine_damage,
        gear_box_damage,
        vehicle_fia_flags,
        ers_store_energy,
        ers_deploy_mode,
        ers_harvested_this_lap_mguk,
        ers_harvested_this_lap_mguh,
        ers_deployed_this_lap,
    ))
}

pub fn parse_car_status_data<T: BufRead>(
    mut reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketCarStatusData, UnpackError> {
    assert_packet_size(size, CAR_STATUS_PACKET_SIZE)?;

    let mut car_status_data = Vec::with_capacity(NUMBER_CARS);
    for _ in 0..NUMBER_CARS {
        let csd = parse_car(&mut reader)?;
        car_status_data.push(csd);
    }

    Ok(PacketCarStatusData::new(header, car_status_data))
}
