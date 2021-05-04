use std::io::BufRead;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::packet::car_setup::{CarSetupData, PacketCarSetupData};
use crate::packet::generic::WheelData;
use crate::packet::header::PacketHeader;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;

fn parse_car_setup<T: BufRead>(reader: &mut T) -> CarSetupData {
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
    let tyre_pressures = WheelData::new(
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
        reader.read_f32::<LittleEndian>().unwrap(),
    );
    let ballast = reader.read_u8().unwrap();
    let fuel_load = reader.read_f32::<LittleEndian>().unwrap();

    CarSetupData::from_2020(
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
        tyre_pressures,
        ballast,
        fuel_load,
    )
}

pub(crate) fn parse_car_setup_data<T: BufRead>(
    mut reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketCarSetupData, UnpackError> {
    assert_packet_size(size, CAR_SETUPS_PACKET_SIZE)?;

    let mut car_setups = Vec::with_capacity(NUMBER_CARS);
    for _ in 0..NUMBER_CARS {
        let csd = parse_car_setup(&mut reader);
        car_setups.push(csd);
    }

    Ok(PacketCarSetupData::new(header, car_setups))
}
