use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::BufRead;

use crate::packet::car_telemetry::{CarTelemetryData, PacketCarTelemetryData, SurfaceType};
use crate::packet::generic::WheelData;
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;

impl TryFrom<u8> for SurfaceType {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
            _ => Err(UnpackError(format!("Invalid SurfaceType value: {}", value))),
        }
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
        SurfaceType::try_from(reader.read_u8().unwrap())?,
        SurfaceType::try_from(reader.read_u8().unwrap())?,
        SurfaceType::try_from(reader.read_u8().unwrap())?,
        SurfaceType::try_from(reader.read_u8().unwrap())?,
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
) -> Result<PacketCarTelemetryData, UnpackError> {
    let mut car_telemetry_data = Vec::with_capacity(20);
    for _ in 0..20 {
        let ctd = parse_car_telemetry(&mut reader)?;
        car_telemetry_data.push(ctd);
    }

    let button_status = reader.read_u32::<LittleEndian>().unwrap();

    Ok(PacketCarTelemetryData::new(
        header,
        car_telemetry_data,
        button_status,
    ))
}
