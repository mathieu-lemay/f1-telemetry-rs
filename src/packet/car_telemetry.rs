use byteorder::{LittleEndian, ReadBytesExt};
use getset::Getters;
use std::convert::TryFrom;
use std::io::BufRead;

use super::header::PacketHeader;
use crate::packet::UnpackError;

#[derive(Debug)]
pub enum SurfaceType {
    Tarmac,
    RumbleStrip,
    Concrete,
    Rock,
    Gravel,
    Mud,
    Sand,
    Grass,
    Water,
    Cobblestone,
    Metal,
    Ridged,
}

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

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Temperatures {
    rear_left: u16,
    rear_right: u16,
    front_left: u16,
    front_right: u16,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct TyrePressures {
    rear_left: f32,
    rear_right: f32,
    front_left: f32,
    front_right: f32,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct SurfaceTypes {
    rear_left: SurfaceType,
    rear_right: SurfaceType,
    front_left: SurfaceType,
    front_right: SurfaceType,
}

/// This type is used for the 20-element 'car_telemetry_data' array of the [`PacketCarTelemetryData`] type.
///
/// ## Specification
/// speed                     Speed of car in kilometres per hour
/// throttle                  Amount of throttle applied (0.0 to 1.0)
/// steer                     Steering (-1.0 (full lock left) to 1.0 (full lock right))
/// brake                     Amount of brake applied (0 to 1.0)
/// clutch                    Amount of clutch applied (0 to 100)
/// gear                      Gear selected (1-8, N=0, R=-1)
/// engine_rpm                Engine RPM
/// drs                       0 = off, 1 = on
/// rev_lights_percent        Rev lights indicator (percentage)
/// brakes_temperature        Brakes temperature (celsius)
/// tyres_surface_temperature Tyres surface temperature (celsius)
/// tyres_inner_temperature   Tyres inner temperature (celsius)
/// engine_temperature        Engine temperature (celsius)
/// tyre_pressures            Tyres pressure (PSI)
/// surface_type              Driving surface, see appendices
///
/// [`PacketCarTelemetryData`]: ./struct.CarTelemetryData.html
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CarTelemetryData {
    speed: u16,
    throttle: f32,
    steer: f32,
    brake: f32,
    clutch: u8,
    gear: i8,
    engine_rpm: u16,
    drs: bool,
    rev_lights_percent: u8,
    brakes_temperature: Temperatures,
    tyres_surface_temperature: Temperatures,
    tyres_inner_temperature: Temperatures,
    engine_temperature: u16,
    tyre_pressures: TyrePressures,
    surface_types: SurfaceTypes,
}

impl CarTelemetryData {
    pub fn new<T: BufRead>(reader: &mut T) -> Result<CarTelemetryData, UnpackError> {
        let speed = reader.read_u16::<LittleEndian>().unwrap();
        let throttle = reader.read_f32::<LittleEndian>().unwrap();
        let steer = reader.read_f32::<LittleEndian>().unwrap();
        let brake = reader.read_f32::<LittleEndian>().unwrap();
        let clutch = reader.read_u8().unwrap();
        let gear = reader.read_i8().unwrap();
        let engine_rpm = reader.read_u16::<LittleEndian>().unwrap();
        let drs = reader.read_u8().unwrap() == 1;
        let rev_lights_percent = reader.read_u8().unwrap();
        let brakes_temperature = Temperatures {
            rear_left: reader.read_u16::<LittleEndian>().unwrap(),
            rear_right: reader.read_u16::<LittleEndian>().unwrap(),
            front_left: reader.read_u16::<LittleEndian>().unwrap(),
            front_right: reader.read_u16::<LittleEndian>().unwrap(),
        };
        let tyres_surface_temperature = Temperatures {
            rear_left: reader.read_u16::<LittleEndian>().unwrap(),
            rear_right: reader.read_u16::<LittleEndian>().unwrap(),
            front_left: reader.read_u16::<LittleEndian>().unwrap(),
            front_right: reader.read_u16::<LittleEndian>().unwrap(),
        };
        let tyres_inner_temperature = Temperatures {
            rear_left: reader.read_u16::<LittleEndian>().unwrap(),
            rear_right: reader.read_u16::<LittleEndian>().unwrap(),
            front_left: reader.read_u16::<LittleEndian>().unwrap(),
            front_right: reader.read_u16::<LittleEndian>().unwrap(),
        };
        let engine_temperature = reader.read_u16::<LittleEndian>().unwrap();
        let tyre_pressures = TyrePressures {
            rear_left: reader.read_f32::<LittleEndian>().unwrap(),
            rear_right: reader.read_f32::<LittleEndian>().unwrap(),
            front_left: reader.read_f32::<LittleEndian>().unwrap(),
            front_right: reader.read_f32::<LittleEndian>().unwrap(),
        };
        let surface_types = SurfaceTypes {
            rear_left: SurfaceType::try_from(reader.read_u8().unwrap())?,
            rear_right: SurfaceType::try_from(reader.read_u8().unwrap())?,
            front_left: SurfaceType::try_from(reader.read_u8().unwrap())?,
            front_right: SurfaceType::try_from(reader.read_u8().unwrap())?,
        };

        Ok(CarTelemetryData {
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
        })
    }
}

/// Bit-mask values for the 'button_status' field in [`PacketCarTelemetryDataCar`]
///
/// [`PacketCarTelemetryData`]: ./struct.CarTelemetryData.html
#[derive(Debug)]
pub enum ButtonFlag {
    Cross = 0x0001,
    Triangle = 0x0002,
    Circle = 0x0004,
    Square = 0x0008,
    DPadLeft = 0x0010,
    DPadRight = 0x0020,
    DPadUp = 0x0040,
    DPadDown = 0x0080,
    Options = 0x0100,
    L1 = 0x0200,
    R1 = 0x0400,
    L2 = 0x0800,
    R2 = 0x1000,
    LeftStickClick = 0x2000,
    RightStickClick = 0x4000,
}

/// This packet details telemetry for all the cars in the race.
///
/// It details various values that would be recorded on the car such as speed, throttle application, DRS etc.
///
/// Frequency: Rate as specified in menus
///
/// Size: 1347 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// header:             Header
/// car_telemetry_data: List of car telemetry (20)
/// button_status:      Bit flags specifying which buttons are being
///                     pressed currently - see appendices
/// ```
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct PacketCarTelemetryData {
    header: PacketHeader,
    car_telemetry_data: Vec<CarTelemetryData>,
    button_status: u32,
}

impl PacketCarTelemetryData {
    pub fn new<T: BufRead>(
        mut reader: &mut T,
        header: PacketHeader,
    ) -> Result<PacketCarTelemetryData, UnpackError> {
        let mut car_telemetry_data = Vec::with_capacity(20);
        for _ in 0..20 {
            let ctd = CarTelemetryData::new(&mut reader)?;
            car_telemetry_data.push(ctd);
        }

        let button_status = reader.read_u32::<LittleEndian>().unwrap();

        Ok(PacketCarTelemetryData {
            header,
            car_telemetry_data,
            button_status,
        })
    }
}
