use byteorder::{LittleEndian, ReadBytesExt};
use std::io::BufRead;

use crate::f1_2020::generic::unpack_flag;
use crate::packet::header::PacketHeader;
use crate::packet::session::*;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

const PACKET_SIZE: usize = 251;

fn unpack_weather(value: u8) -> Result<Weather, UnpackError> {
    match value {
        0 => Ok(Weather::Clear),
        1 => Ok(Weather::LightCloud),
        2 => Ok(Weather::Overcast),
        3 => Ok(Weather::LightRain),
        4 => Ok(Weather::HeavyRain),
        5 => Ok(Weather::Storm),
        _ => Err(UnpackError(format!("Invalid Weather value: {}", value))),
    }
}

fn unpack_session_type(value: u8) -> Result<SessionType, UnpackError> {
    match value {
        0 => Ok(SessionType::Unknown),
        1 => Ok(SessionType::Practice1),
        2 => Ok(SessionType::Practice2),
        3 => Ok(SessionType::Practice3),
        4 => Ok(SessionType::PracticeShort),
        5 => Ok(SessionType::Qualifying1),
        6 => Ok(SessionType::Qualifying2),
        7 => Ok(SessionType::Qualifying3),
        8 => Ok(SessionType::QualifyingShort),
        9 => Ok(SessionType::OneShotQualifying),
        10 => Ok(SessionType::Race),
        11 => Ok(SessionType::Race2),
        12 => Ok(SessionType::TimeTrial),
        _ => Err(UnpackError(format!("Invalid SessionType value: {}", value))),
    }
}

fn unpack_track(value: i8) -> Result<Track, UnpackError> {
    match value {
        0 => Ok(Track::Melbourne),
        1 => Ok(Track::PaulRicard),
        2 => Ok(Track::Shanghai),
        3 => Ok(Track::Sakhir),
        4 => Ok(Track::Catalunya),
        5 => Ok(Track::Monaco),
        6 => Ok(Track::Montreal),
        7 => Ok(Track::Silverstone),
        8 => Ok(Track::Hockenheim),
        9 => Ok(Track::Hungaroring),
        10 => Ok(Track::Spa),
        11 => Ok(Track::Monza),
        12 => Ok(Track::Singapore),
        13 => Ok(Track::Suzuka),
        14 => Ok(Track::AbuDhabi),
        15 => Ok(Track::Texas),
        16 => Ok(Track::Brazil),
        17 => Ok(Track::Austria),
        18 => Ok(Track::Sochi),
        19 => Ok(Track::Mexico),
        20 => Ok(Track::Baku),
        21 => Ok(Track::SakhirShort),
        22 => Ok(Track::SilverstoneShort),
        23 => Ok(Track::TexasShort),
        24 => Ok(Track::SuzukaShort),
        25 => Ok(Track::Hanoi),
        26 => Ok(Track::Zandvoort),
        -1 => Ok(Track::Unknown),
        _ => Err(UnpackError(format!("Invalid Track value: {}", value))),
    }
}

fn unpack_formula(value: u8) -> Result<Formula, UnpackError> {
    match value {
        0 => Ok(Formula::F1Modern),
        1 => Ok(Formula::F1Classic),
        2 => Ok(Formula::F2),
        3 => Ok(Formula::F1Generic),
        _ => Err(UnpackError(format!("Invalid Formula value: {}", value))),
    }
}

fn unpack_safety_car(value: u8) -> Result<SafetyCar, UnpackError> {
    match value {
        0 => Ok(SafetyCar::None),
        1 => Ok(SafetyCar::Full),
        2 => Ok(SafetyCar::Virtual),
        _ => Err(UnpackError(format!("Invalid SafetyCar value: {}", value))),
    }
}

fn parse_marshal_zone<T: BufRead>(reader: &mut T) -> Result<MarshalZone, UnpackError> {
    let zone_start = reader.read_f32::<LittleEndian>().unwrap();
    let zone_flag = unpack_flag(reader.read_i8().unwrap())?;

    Ok(MarshalZone::new(zone_start, zone_flag))
}

fn parse_weather_forecast_sample<T: BufRead>(
    reader: &mut T,
) -> Result<WeatherForecastSample, UnpackError> {
    let session_type = unpack_session_type(reader.read_u8().unwrap())?;
    let time_offset = reader.read_u8().unwrap();
    let weather = unpack_weather(reader.read_u8().unwrap())?;
    let track_temperature = reader.read_i8().unwrap();
    let air_temperature = reader.read_i8().unwrap();

    Ok(WeatherForecastSample::new(
        session_type,
        time_offset,
        weather,
        track_temperature,
        air_temperature,
    ))
}

pub(crate) fn parse_session_data<T: BufRead>(
    mut reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketSessionData, UnpackError> {
    assert_packet_size(size, PACKET_SIZE)?;

    let weather = unpack_weather(reader.read_u8().unwrap())?;
    let track_temperature = reader.read_i8().unwrap();
    let air_temperature = reader.read_i8().unwrap();
    let total_laps = reader.read_u8().unwrap();
    let track_length = reader.read_u16::<LittleEndian>().unwrap();
    let session_type = unpack_session_type(reader.read_u8().unwrap())?;
    let track = unpack_track(reader.read_i8().unwrap())?;
    let formula = unpack_formula(reader.read_u8().unwrap())?;
    let session_time_left = reader.read_u16::<LittleEndian>().unwrap();
    let session_duration = reader.read_u16::<LittleEndian>().unwrap();
    let pit_speed_limit = reader.read_u8().unwrap();
    let game_paused = reader.read_u8().unwrap();
    let is_spectating = reader.read_u8().unwrap() == 1;
    let spectator_car_index = reader.read_u8().unwrap();
    let sli_pro_native_support = reader.read_u8().unwrap() == 1;

    let num_marshal_zones = reader.read_u8().unwrap();
    let mut marshal_zones = Vec::with_capacity(21);
    for _ in 0..21 {
        let mz = parse_marshal_zone(&mut reader)?;
        marshal_zones.push(mz);
    }

    let safety_car_status = unpack_safety_car(reader.read_u8().unwrap())?;
    let network_game = reader.read_u8().unwrap() == 1;

    let num_weather_forecast_samples = reader.read_u8().unwrap();
    let mut weather_forecast_samples = Vec::with_capacity(20);
    for _ in 0..20 {
        let wfs = parse_weather_forecast_sample(&mut reader)?;
        weather_forecast_samples.push(wfs);
    }

    Ok(PacketSessionData::from_2020(
        header,
        weather,
        track_temperature,
        air_temperature,
        total_laps,
        track_length,
        session_type,
        track,
        formula,
        session_time_left,
        session_duration,
        pit_speed_limit,
        game_paused,
        is_spectating,
        spectator_car_index,
        sli_pro_native_support,
        num_marshal_zones,
        marshal_zones,
        safety_car_status,
        network_game,
        num_weather_forecast_samples,
        weather_forecast_samples,
    ))
}
