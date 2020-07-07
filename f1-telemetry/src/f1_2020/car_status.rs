use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::BufRead;

use crate::packet::car_status::{
    CarStatusData, ERSDeployMode, FuelMix, PacketCarStatusData, TractionControl, TyreCompound,
    TyreCompoundVisual, DRS,
};
use crate::packet::generic::{Flag, WheelData};
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

const PACKET_SIZE: usize = 1344;

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
    let vehicle_fia_flags = Flag::try_from(reader.read_i8().unwrap())?;
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
    assert_packet_size(size, PACKET_SIZE)?;

    let mut car_status_data = Vec::with_capacity(22);
    for _ in 0..22 {
        let csd = parse_car(&mut reader)?;
        car_status_data.push(csd);
    }

    Ok(PacketCarStatusData::new(header, car_status_data))
}
