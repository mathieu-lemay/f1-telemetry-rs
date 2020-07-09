use std::io::BufRead;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::packet::car_telemetry::{CarTelemetryData, PacketCarTelemetryData, SurfaceType};
use crate::packet::generic::WheelData;
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

fn unpack_surface_type(value: u8) -> Result<SurfaceType, UnpackError> {
    match value {
        0 => Ok(SurfaceType::Tarmac),
        1 => Ok(SurfaceType::RumbleStrip),
        2 => Ok(SurfaceType::Concrete),
        3 => Ok(SurfaceType::Rock),
        4 => Ok(SurfaceType::Gravel),
        5 => Ok(SurfaceType::Mud),
        6 => Ok(SurfaceType::Sand),
        7 => Ok(SurfaceType::Grass),
        8 => Ok(SurfaceType::Water),
        9 => Ok(SurfaceType::Cobblestone),
        10 => Ok(SurfaceType::Metal),
        11 => Ok(SurfaceType::Ridged),
        12 => Ok(SurfaceType::Unknown),
        _ => Err(UnpackError(format!("Invalid SurfaceType value: {}", value))),
    }
}

fn parse_car_telemetry<T: BufRead>(reader: &mut T) -> Result<CarTelemetryData, UnpackError> {
    let speed = reader.read_u16::<LittleEndian>().unwrap();
    let throttle = reader.read_f32::<LittleEndian>().unwrap();
    let steer = reader.read_f32::<LittleEndian>().unwrap();
    let brake = reader.read_f32::<LittleEndian>().unwrap();
    let clutch = reader.read_u8().unwrap();
    let gear = reader.read_i8().unwrap();
    let engine_rpm = reader.read_u16::<LittleEndian>().unwrap();
    let drs = reader.read_u8().unwrap() == 1;
    let rev_lights_percent = reader.read_u8().unwrap();
    let brakes_temperature = WheelData::new(
        reader.read_u16::<LittleEndian>().unwrap(),
        reader.read_u16::<LittleEndian>().unwrap(),
        reader.read_u16::<LittleEndian>().unwrap(),
        reader.read_u16::<LittleEndian>().unwrap(),
    );
    let tyres_surface_temperature = WheelData::new(
        reader.read_u16::<LittleEndian>().unwrap(),
        reader.read_u16::<LittleEndian>().unwrap(),
        reader.read_u16::<LittleEndian>().unwrap(),
        reader.read_u16::<LittleEndian>().unwrap(),
    );
    let tyres_inner_temperature = WheelData::new(
        reader.read_u16::<LittleEndian>().unwrap(),
        reader.read_u16::<LittleEndian>().unwrap(),
        reader.read_u16::<LittleEndian>().unwrap(),
        reader.read_u16::<LittleEndian>().unwrap(),
    );
    let engine_temperature = reader.read_u16::<LittleEndian>().unwrap();
    let tyre_pressures = WheelData::new(
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
    );
    let surface_types = WheelData::new(
        unpack_surface_type(reader.read_u8().unwrap())?,
        unpack_surface_type(reader.read_u8().unwrap())?,
        unpack_surface_type(reader.read_u8().unwrap())?,
        unpack_surface_type(reader.read_u8().unwrap())?,
    );

    Ok(CarTelemetryData::new(
        speed,
        throttle,
        steer,
        brake,
        clutch,
        gear,
        engine_rpm,
        drs,
        rev_lights_percent,
        brakes_temperature,
        tyres_surface_temperature,
        tyres_inner_temperature,
        engine_temperature,
        tyre_pressures,
        surface_types,
    ))
}

pub(crate) fn parse_car_telemetry_data<T: BufRead>(
    mut reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketCarTelemetryData, UnpackError> {
    assert_packet_size(size, CAR_TELEMETRY_PACKET_SIZE)?;

    let mut car_telemetry_data = Vec::with_capacity(NUMBER_CARS);
    for _ in 0..NUMBER_CARS {
        let ctd = parse_car_telemetry(&mut reader)?;
        car_telemetry_data.push(ctd);
    }

    let button_status = reader.read_u32::<LittleEndian>().unwrap();

    Ok(PacketCarTelemetryData::from_2019(
        header,
        car_telemetry_data,
        button_status,
    ))
}
