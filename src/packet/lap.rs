use super::header::PacketHeader;
use crate::packet::UnpackError;
use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::BufRead;

#[derive(Debug)]
pub enum PitStatus {
    None,
    Pitting,
    PitLane,
}

impl TryFrom<u8> for PitStatus {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PitStatus::None),
            1 => Ok(PitStatus::Pitting),
            2 => Ok(PitStatus::PitLane),
            _ => Err(UnpackError(format!("Invalid PitStatus value: {}", value))),
        }
    }
}

#[derive(Debug)]
pub enum DriverStatus {
    Garage,
    FlyingLap,
    InLap,
    OutLap,
    OnTrack,
}

impl TryFrom<u8> for DriverStatus {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DriverStatus::Garage),
            1 => Ok(DriverStatus::FlyingLap),
            2 => Ok(DriverStatus::InLap),
            3 => Ok(DriverStatus::OutLap),
            4 => Ok(DriverStatus::OnTrack),
            _ => Err(UnpackError(format!(
                "Invalid DriverStatus value: {}",
                value
            ))),
        }
    }
}

#[derive(Debug)]
pub enum ResultStatus {
    Invalid,
    Inactive,
    Active,
    Finished,
    Disqualified,
    NotClassified,
    Retired,
}

impl TryFrom<u8> for ResultStatus {
    type Error = UnpackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ResultStatus::Invalid),
            1 => Ok(ResultStatus::Inactive),
            2 => Ok(ResultStatus::Active),
            3 => Ok(ResultStatus::Finished),
            4 => Ok(ResultStatus::Disqualified),
            5 => Ok(ResultStatus::NotClassified),
            6 => Ok(ResultStatus::Retired),
            _ => Err(UnpackError(format!(
                "Invalid ResultStatus value: {}",
                value
            ))),
        }
    }
}

/// This type is used for the 20-element `lap_data` array of the [`PacketLapData`] type.
///
/// Size: 41 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// last_lap_time:       Last lap time in seconds
/// current_lap_time:    Current time around the lap in seconds
/// best_lap_time:       Best lap time of the session in seconds
/// sector_1_time:       Sector 1 time in seconds
/// sector_2_time:       Sector 2 time in seconds
/// lap_distance:        Distance vehicle is around current lap in metres – could
///                      be negative if line hasn’t been crossed yet
/// total_distance:      Total distance travelled in session in metres – could
///                      be negative if line hasn’t been crossed yet
/// safety_car_delta:    Delta in seconds for safety car
/// car_position:        Car race position
/// current_lap_num:     Current lap number
/// pit_status:          0 = none, 1 = pitting, 2 = in pit area
/// sector:              0 = sector1, 1 = sector2, 2 = sector3
/// current_lap_invalid: Current lap invalid - 0 = valid, 1 = invalid
/// penalties:           Accumulated time penalties in seconds to be added
/// grid_position:       Grid position the vehicle started the race in
/// driver_status:       Status of driver - 0 = in garage, 1 = flying lap
///                      2 = in lap, 3 = out lap, 4 = on track
/// result_status:       Result status - 0 = invalid, 1 = inactive, 2 = active
///                      3 = finished, 4 = disqualified, 5 = not classified
///                      6 = retired
/// ```
/// [`PacketLapData`]: ./struct.PacketLapData.html
#[derive(Debug)]
pub struct LapData {
    pub last_lap_time: f32,
    pub current_lap_time: f32,
    pub best_lap_time: f32,
    pub sector_1_time: f32,
    pub sector_2_time: f32,
    pub lap_distance: f32,
    pub total_distance: f32,
    pub safety_car_delta: f32,
    pub car_position: u8,
    pub current_lap_num: u8,
    pub pit_status: PitStatus,
    pub sector: u8,
    pub current_lap_invalid: u8,
    pub penalties: u8,
    pub grid_position: u8,
    pub driver_status: DriverStatus,
    pub result_status: ResultStatus,
}

impl LapData {
    pub fn new<T: BufRead>(reader: &mut T) -> Result<LapData, UnpackError> {
        let last_lap_time = reader.read_f32::<LittleEndian>().unwrap();
        let current_lap_time = reader.read_f32::<LittleEndian>().unwrap();
        let best_lap_time = reader.read_f32::<LittleEndian>().unwrap();
        let sector_1_time = reader.read_f32::<LittleEndian>().unwrap();
        let sector_2_time = reader.read_f32::<LittleEndian>().unwrap();
        let lap_distance = reader.read_f32::<LittleEndian>().unwrap();
        let total_distance = reader.read_f32::<LittleEndian>().unwrap();
        let safety_car_delta = reader.read_f32::<LittleEndian>().unwrap();
        let car_position = reader.read_u8().unwrap();
        let current_lap_num = reader.read_u8().unwrap();
        let pit_status = PitStatus::try_from(reader.read_u8().unwrap())?;
        let sector = reader.read_u8().unwrap();
        let current_lap_invalid = reader.read_u8().unwrap();
        let penalties = reader.read_u8().unwrap();
        let grid_position = reader.read_u8().unwrap();
        let driver_status = DriverStatus::try_from(reader.read_u8().unwrap())?;
        let result_status = ResultStatus::try_from(reader.read_u8().unwrap())?;

        Ok(LapData {
            last_lap_time,
            current_lap_time,
            best_lap_time,
            sector_1_time,
            sector_2_time,
            lap_distance,
            total_distance,
            safety_car_delta,
            car_position,
            current_lap_num,
            pit_status,
            sector,
            current_lap_invalid,
            penalties,
            grid_position,
            driver_status,
            result_status,
        })
    }
}

/// The lap data packet gives details of all the cars in the session.
///
/// Frequency: Rate as specified in menus
///
/// Size: 843 bytes
///
/// Version: 1
///
/// ## Specification
/// ```text
/// header:   Header
/// lap_data: Lap data for all cars on track
/// ```
#[derive(Debug)]
pub struct PacketLapData {
    pub header: PacketHeader,
    pub lap_data: Vec<LapData>,
}

impl PacketLapData {
    pub fn new<T: BufRead>(
        mut reader: &mut T,
        header: PacketHeader,
    ) -> Result<PacketLapData, UnpackError> {
        let mut lap_data = Vec::with_capacity(20);

        for _ in 0..20 {
            let ld = LapData::new(&mut reader)?;
            lap_data.push(ld);
        }

        Ok(PacketLapData { header, lap_data })
    }
}
