use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::BufRead;

use crate::packet::header::PacketHeader;
use crate::packet::lap::{DriverStatus, LapData, PacketLapData, PitStatus, ResultStatus};
use crate::packet::UnpackError;

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

fn parse_lap<T: BufRead>(reader: &mut T) -> Result<LapData, UnpackError> {
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
    let current_lap_invalid = reader.read_u8().unwrap() == 1;
    let penalties = reader.read_u8().unwrap();
    let grid_position = reader.read_u8().unwrap();
    let driver_status = DriverStatus::try_from(reader.read_u8().unwrap())?;
    let result_status = ResultStatus::try_from(reader.read_u8().unwrap())?;

    Ok(LapData::new(
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
    ))
}

pub(crate) fn parse_lap_data<T: BufRead>(
    mut reader: &mut T,
    header: PacketHeader,
) -> Result<PacketLapData, UnpackError> {
    let mut lap_data = Vec::with_capacity(20);

    for _ in 0..20 {
        let ld = parse_lap(&mut reader)?;
        lap_data.push(ld);
    }

    Ok(PacketLapData::new(header, lap_data))
}
