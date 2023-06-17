use std::io::BufRead;

use serde::Deserialize;

use crate::packet::generic::SessionType;
use crate::packet::header::PacketHeader;
use crate::packet::session::*;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;
use super::generic::unpack_flag;

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

/// The session packet includes details about the current session in progress.
///
/// Frequency: 2 per second
/// Size: 149 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:                 Header
/// weather:                Weather - 0 = clear, 1 = light cloud, 2 = overcast
///                         3 = light rain, 4 = heavy rain, 5 = storm
/// track_temperature:      Track temp. in degrees celsius
/// air_temperature:        Air temp. in degrees celsius
/// total_laps:             Total number of laps in this race
/// track_length:           Track length in metres
/// session_type:           0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
///                         5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
///                         10 = R, 11 = R2, 12 = Time Trial
/// track_id:               -1 for unknown, 0-21 for tracks, see appendix
/// formula:                Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2,
///                         3 = F1 Generic
/// session_time_left:      Time left in session in seconds
/// session_duration:       Session duration in seconds
/// pit_speed_limit:        Pit speed limit in kilometres per hour
/// game_paused:            Whether the game is paused
/// is_spectating:          Whether the player is spectating
/// spectator_car_index:    Index of the car being spectated
/// sli_pro_native_support: SLI Pro support, 0 = inactive, 1 = active
/// num_marshal_zones:      Number of marshal zones to follow
/// marshal_zones:          List of marshal zones – max 21
/// safety_car_status:      0 = no safety car, 1 = full safety car
///                         2 = virtual safety car
/// network_game:           0 = offline, 1 = online
/// ```
#[derive(Deserialize)]
struct RawSessionData {
    weather: u8,
    track_temperature: i8,
    air_temperature: i8,
    total_laps: u8,
    track_length: u16,
    session_type: u8,
    track: i8,
    formula: u8,
    session_time_left: u16,
    session_duration: u16,
    pit_speed_limit: u8,
    game_paused: bool,
    is_spectating: bool,
    spectator_car_index: u8,
    sli_pro_native_support: bool,
    num_marshal_zones: u8,
    marshal_zones: [RawMarshalZone; NUMBER_MARSHAL_ZONES],
    safety_car_status: u8,
    network_game: bool,
}

/// Description of a marshal zone
///
/// ## Specification
/// ```text
/// zone_start: Fraction (0..1) of way through the lap the marshal zone starts
/// zone_flag:  -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
/// ```
#[derive(Deserialize)]
struct RawMarshalZone {
    zone_start: f32,
    zone_flag: i8,
}

impl TryFrom<&RawMarshalZone> for MarshalZone {
    type Error = UnpackError;

    fn try_from(mz: &RawMarshalZone) -> Result<Self, Self::Error> {
        let zone_flag = unpack_flag(mz.zone_flag)?;

        Ok(MarshalZone {
            zone_start: mz.zone_start,
            zone_flag,
        })
    }
}

pub(crate) fn parse_session_data<T: BufRead>(
    reader: &mut T,
    header: PacketHeader,
    size: usize,
) -> Result<PacketSessionData, UnpackError> {
    assert_packet_size(size, SESSION_PACKET_SIZE)?;

    let session_data: RawSessionData = bincode::deserialize_from(reader)?;

    let weather = unpack_weather(session_data.weather)?;
    let session_type = unpack_session_type(session_data.session_type)?;
    let track = unpack_track(session_data.track)?;
    let formula = unpack_formula(session_data.formula)?;
    let marshal_zones: Vec<MarshalZone> = session_data
        .marshal_zones
        .iter()
        .map(|mz| mz.try_into())
        .collect::<Result<Vec<MarshalZone>, UnpackError>>()?;
    let safety_car_status = unpack_safety_car(session_data.safety_car_status)?;

    Ok(PacketSessionData {
        header,
        weather,
        track_temperature: session_data.track_temperature,
        air_temperature: session_data.air_temperature,
        total_laps: session_data.total_laps,
        track_length: session_data.track_length,
        session_type,
        track,
        formula,
        session_time_left: session_data.session_time_left,
        session_duration: session_data.session_duration,
        pit_speed_limit: session_data.pit_speed_limit,
        game_paused: session_data.game_paused,
        is_spectating: session_data.is_spectating,
        spectator_car_index: session_data.spectator_car_index,
        sli_pro_native_support: session_data.sli_pro_native_support,
        num_marshal_zones: session_data.num_marshal_zones,
        marshal_zones,
        safety_car_status,
        network_game: session_data.network_game,
        weather_forecast: None,
        ai_difficulty: None,
        season_identifier: None,
        weekend_identifier: None,
        session_identifier: None,
        pit_stop_window_ideal_lap: None,
        pit_stop_window_latest_lap: None,
        pit_stop_rejoin_position: None,
        driving_assists: None,
        game_mode: None,
        rule_set: None,
        time_of_day: None,
        session_length: None,
        speed_units_lead_player: None,
        temperature_units_lead_player: None,
        speed_units_secondary_player: None,
        temperature_units_secondary_player: None,
        num_safety_car_periods: None,
        num_virtual_safety_car_periods: None,
        num_red_flag_periods: None,
    })
}
