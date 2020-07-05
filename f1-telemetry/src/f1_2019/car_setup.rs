use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::BufRead;

use crate::packet::car_setup::{CarSetupData, PacketCarSetupData, TractionControl};
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;

impl TryFrom<u8> for TractionControl {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
}

fn parse_car_setup<T: BufRead>(reader: &mut T) -> Result<CarSetupData, UnpackError> {
    let front_wing = reader.read_u8().unwrap();
    let rear_wing = reader.read_u8().unwrap();
    let on_throttle = reader.read_u8().unwrap();
    let off_throttle = reader.read_u8().unwrap();
    let front_camber = reader.read_f32::<LittleEndian>().unwrap();
    let rear_camber = reader.read_f32::<LittleEndian>().unwrap();
    let front_toe = reader.read_f32::<LittleEndian>().unwrap();
    let rear_toe = reader.read_f32::<LittleEndian>().unwrap();
    let front_suspension = reader.read_u8().unwrap();
    let rear_suspension = reader.read_u8().unwrap();
    let front_anti_roll_bar = reader.read_u8().unwrap();
    let rear_anti_roll_bar = reader.read_u8().unwrap();
    let front_suspension_height = reader.read_u8().unwrap();
    let rear_suspension_height = reader.read_u8().unwrap();
    let brake_pressure = reader.read_u8().unwrap();
    let brake_bias = reader.read_u8().unwrap();
    let front_tyre_pressure = reader.read_f32::<LittleEndian>().unwrap();
    let rear_tyre_pressure = reader.read_f32::<LittleEndian>().unwrap();
    let ballast = reader.read_u8().unwrap();
    let fuel_load = reader.read_f32::<LittleEndian>().unwrap();

    Ok(CarSetupData::new(
        front_wing,
        rear_wing,
        on_throttle,
        off_throttle,
        front_camber,
        rear_camber,
        front_toe,
        rear_toe,
        front_suspension,
        rear_suspension,
        front_anti_roll_bar,
        rear_anti_roll_bar,
        front_suspension_height,
        rear_suspension_height,
        brake_pressure,
        brake_bias,
        front_tyre_pressure,
        rear_tyre_pressure,
        ballast,
        fuel_load,
    ))
}

pub(crate) fn parse_car_setup_data<T: BufRead>(
    mut reader: &mut T,
    header: PacketHeader,
) -> Result<PacketCarSetupData, UnpackError> {
    let mut car_setups = Vec::with_capacity(20);
    for _ in 0..20 {
        let csd = parse_car_setup(&mut reader)?;
        car_setups.push(csd);
    }

    Ok(PacketCarSetupData::new(header, car_setups))
}
