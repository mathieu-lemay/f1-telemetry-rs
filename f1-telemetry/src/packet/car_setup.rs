use byteorder::{LittleEndian, ReadBytesExt};
use getset::{CopyGetters, Getters};
use std::convert::TryFrom;
use std::io::BufRead;

use super::header::PacketHeader;
use crate::packet::UnpackError;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TractionControl {
    Off,
    Low,
    High,
}

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

/// This type is used for the 20-element `car_setups` array of the [`PacketCarSetupData`] type.
///
/// ## Specification
/// front_wing              Front wing aero
/// rear_wing               Rear wing aero
/// on_throttle             Differential adjustment on throttle (percentage)
/// off_throttle            Differential adjustment off throttle (percentage)
/// front_camber            Front camber angle (suspension geometry)
/// rear_camber             Rear camber angle (suspension geometry)
/// front_toe               Front toe angle (suspension geometry)
/// rear_toe                Rear toe angle (suspension geometry)
/// front_suspension        Front suspension
/// rear_suspension         Rear suspension
/// front_anti_roll_bar     Front anti-roll bar
/// rear_anti_roll_bar      Rear anti-roll bar
/// front_suspension_height Front ride height
/// rear_suspension_height  Rear ride height
/// brake_pressure          Brake pressure (percentage)
/// brake_bias              Brake bias (percentage)
/// front_tyre_pressure     Front tyre pressure (PSI)
/// rear_tyre_pressure      Rear tyre pressure (PSI)
/// ballast                 Ballast
/// fuel_load               Fuel load
///
/// [`PacketCarSetupData`]: ./struct.CarSetupData.html
#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct CarSetupData {
    front_wing: u8,
    rear_wing: u8,
    on_throttle: u8,
    off_throttle: u8,
    front_camber: f32,
    rear_camber: f32,
    front_toe: f32,
    rear_toe: f32,
    front_suspension: u8,
    rear_suspension: u8,
    front_anti_roll_bar: u8,
    rear_anti_roll_bar: u8,
    front_suspension_height: u8,
    rear_suspension_height: u8,
    brake_pressure: u8,
    brake_bias: u8,
    front_tyre_pressure: f32,
    rear_tyre_pressure: f32,
    ballast: u8,
    fuel_load: f32,
}

impl CarSetupData {
    pub fn new<T: BufRead>(reader: &mut T) -> Result<CarSetupData, UnpackError> {
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

        Ok(CarSetupData {
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
        })
    }
}

/// This packet details the car setups for each vehicle in the session.
///
/// Note that in multiplayer games, other player cars will appear as blank, you will only be able to see your car setup and AI cars.
///
/// Frequency: 2 per second
///
/// Size: 843 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// header:     Header
/// car_setups: List of car setups (20)
/// ```
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct PacketCarSetupData {
    header: PacketHeader,
    car_setups: Vec<CarSetupData>,
}

impl PacketCarSetupData {
    pub fn new<T: BufRead>(
        mut reader: &mut T,
        header: PacketHeader,
    ) -> Result<PacketCarSetupData, UnpackError> {
        let mut car_setups = Vec::with_capacity(20);
        for _ in 0..20 {
            let csd = CarSetupData::new(&mut reader)?;
            car_setups.push(csd);
        }

        Ok(PacketCarSetupData { header, car_setups })
    }
}
