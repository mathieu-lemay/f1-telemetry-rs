use std::io::BufRead;

use serde::Deserialize;

use crate::packet::header::PacketHeader;
use crate::packet::session::*;
use crate::packet::UnpackError;
use crate::utils::assert_packet_size;

use super::consts::*;
use super::generic::unpack_flag;
use super::generic::unpack_session_type;

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
        27 => Ok(Track::Imola),
        28 => Ok(Track::Portimao),
        29 => Ok(Track::Jeddah),
        30 => Ok(Track::Miami),
        31 => Ok(Track::LasVegas),
        32 => Ok(Track::Losail),
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
        4 => Ok(Formula::Beta),
        5 => Ok(Formula::Supercars),
        6 => Ok(Formula::Esports),
        7 => Ok(Formula::F2_21),
        8 => Ok(Formula::F1WorldCar),
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

fn unpack_temperature_change(value: i8) -> Result<TemperatureChange, UnpackError> {
    match value {
        0 => Ok(TemperatureChange::Up),
        1 => Ok(TemperatureChange::Down),
        2 => Ok(TemperatureChange::NoChange),
        _ => Err(UnpackError(format!(
            "Invalid TrackTemperature value: {}",
            value
        ))),
    }
}

fn unpack_forecast_accuracy(value: u8) -> Result<ForecastAccuracy, UnpackError> {
    match value {
        0 => Ok(ForecastAccuracy::Perfect),
        1 => Ok(ForecastAccuracy::Approximate),
        _ => Err(UnpackError(format!(
            "Invalid ForecastAccuracy value: {}",
            value
        ))),
    }
}

fn unpack_braking_assist(value: u8) -> Result<BrakingAssist, UnpackError> {
    match value {
        0 => Ok(BrakingAssist::Off),
        1 => Ok(BrakingAssist::Low),
        2 => Ok(BrakingAssist::Medium),
        3 => Ok(BrakingAssist::High),
        _ => Err(UnpackError(format!(
            "Invalid BrakingAssist value: {}",
            value
        ))),
    }
}

fn unpack_gearbox_assist(value: u8) -> Result<GearboxAssist, UnpackError> {
    match value {
        1 => Ok(GearboxAssist::Manual),
        2 => Ok(GearboxAssist::ManualAndSuggestedGear),
        3 => Ok(GearboxAssist::Automatic),
        _ => Err(UnpackError(format!(
            "Invalid GearboxAssist value: {}",
            value
        ))),
    }
}

fn unpack_dynamic_racing_line(value: u8) -> Result<DynamicRacingLine, UnpackError> {
    match value {
        0 => Ok(DynamicRacingLine::Off),
        1 => Ok(DynamicRacingLine::CornersOnly),
        2 => Ok(DynamicRacingLine::Full),
        _ => Err(UnpackError(format!(
            "Invalid DynamicRacingLine value: {}",
            value
        ))),
    }
}

fn unpack_dynamic_racing_line_type(value: u8) -> Result<DynamicRacingLineType, UnpackError> {
    match value {
        0 => Ok(DynamicRacingLineType::TwoDimensions),
        1 => Ok(DynamicRacingLineType::ThreeDimensions),
        _ => Err(UnpackError(format!(
            "Invalid DynamicRacingLineType value: {}",
            value
        ))),
    }
}

fn unpack_game_mode(value: u8) -> Result<GameMode, UnpackError> {
    match value {
        0 => Ok(GameMode::EventMode),
        3 => Ok(GameMode::GrandPrix),
        4 => Ok(GameMode::GrandPrix23),
        5 => Ok(GameMode::TimeTrial),
        6 => Ok(GameMode::Splitscreen),
        7 => Ok(GameMode::OnlineCustom),
        8 => Ok(GameMode::OnlineLeague),
        11 => Ok(GameMode::CareerInvitational),
        12 => Ok(GameMode::ChampionshipInvitational),
        13 => Ok(GameMode::Championship),
        14 => Ok(GameMode::OnlineChampionship),
        15 => Ok(GameMode::OnlineWeeklyEvent),
        17 => Ok(GameMode::StoryMode),
        19 => Ok(GameMode::Career22),
        20 => Ok(GameMode::Career22Online),
        21 => Ok(GameMode::Career23),
        22 => Ok(GameMode::Career23Online),
        127 => Ok(GameMode::Benchmark),
        _ => Err(UnpackError(format!("Invalid GameMode value: {}", value))),
    }
}

fn unpack_rule_set(value: u8) -> Result<RuleSet, UnpackError> {
    match value {
        0 => Ok(RuleSet::PracticeAndQualifying),
        1 => Ok(RuleSet::Race),
        2 => Ok(RuleSet::TimeTrial),
        4 => Ok(RuleSet::TimeAttack),
        6 => Ok(RuleSet::CheckpointChallenge),
        8 => Ok(RuleSet::Autocross),
        9 => Ok(RuleSet::Drift),
        10 => Ok(RuleSet::AverageSpeedZone),
        11 => Ok(RuleSet::RivalDuel),
        _ => Err(UnpackError(format!("Invalid RuleSet value: {}", value))),
    }
}

fn unpack_session_length(value: u8) -> Result<SessionLength, UnpackError> {
    match value {
        0 => Ok(SessionLength::None),
        2 => Ok(SessionLength::VeryShort),
        3 => Ok(SessionLength::Short),
        4 => Ok(SessionLength::Medium),
        5 => Ok(SessionLength::MediumLong),
        6 => Ok(SessionLength::Long),
        7 => Ok(SessionLength::Full),
        _ => Err(UnpackError(format!(
            "Invalid SessionLength value: {}",
            value
        ))),
    }
}

fn unpack_speed_units(value: u8) -> Result<SpeedUnits, UnpackError> {
    match value {
        0 => Ok(SpeedUnits::MPH),
        1 => Ok(SpeedUnits::KPH),
        _ => Err(UnpackError(format!("Invalid SpeedUnits value: {}", value))),
    }
}

fn unpack_temperature_units(value: u8) -> Result<TemperatureUnits, UnpackError> {
    match value {
        0 => Ok(TemperatureUnits::Celsius),
        1 => Ok(TemperatureUnits::Fahrenheit),
        _ => Err(UnpackError(format!(
            "Invalid TemperatureUnits value: {}",
            value
        ))),
    }
}

/// The session packet includes details about the current session in progress.
///
/// Frequency: 2 per second
/// Size: 644 bytes
/// Version: 1
///
/// ## Specification
/// ```text
/// header:                             Header
/// weather:                            Weather - 0 = clear, 1 = light cloud, 2 = overcast
///                                     3 = light rain, 4 = heavy rain, 5 = storm
/// track_temperature:                  Track temp. in degrees celsius
/// air_temperature:                    Air temp. in degrees celsius
/// total_laps:                         Total number of laps in this race
/// track_length:                       Track length in metres
/// session_type:                       0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P
///                                     5 = Q1, 6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ
///                                     10 = R, 11 = R2, 12 = Time Trial
/// track_id:                           -1 for unknown, 0-21 for tracks, see appendix
/// formula:                            Formula, 0 = F1 Modern, 1 = F1 Classic, 2 = F2,
///                                     3 = F1 Generic
/// session_time_left:                  Time left in session in seconds
/// session_duration:                   Session duration in seconds
/// pit_speed_limit:                    Pit speed limit in kilometres per hour
/// game_paused:                        Whether the game is paused
/// is_spectating:                      Whether the player is spectating
/// spectator_car_index:                Index of the car being spectated
/// sli_pro_native_support:             SLI Pro support, 0 = inactive, 1 = active
/// num_marshal_zones:                  Number of marshal zones to follow
/// marshal_zones:                      List of marshal zones – max 21
/// safety_car_status:                  0 = no safety car, 1 = full safety car
///                                     2 = virtual safety car
/// network_game:                       0 = offline, 1 = online
/// num_weather_forecast_samples:       Number of weather samples to follow
/// weather_forecast_samples:           List of weather forecast samples - max 20
/// forecast_accuracy:                  0 = Perfect, 1 = Approximate
/// ai_difficulty:                      AI Difficulty rating – 0-110
/// season_identifier:                  Identifier for season - persists across saves
/// weekend_identifier:                 Identifier for weekend - persists across saves
/// session_identifier:                 Identifier for session - persists across saves
/// pit_stop_window_ideal_lap:          Ideal lap to pit on for current strategy (player)
/// pit_stop_window_latest_lap:         Latest lap to pit on for current strategy (player)
/// pit_stop_rejoin_position:           Predicted position to rejoin at (player)
/// steering_assist:                    0 = off, 1 = on
/// braking_assist:                     0 = off, 1 = low, 2 = medium, 3 = high
/// gearbox_assist:                     1 = manual, 2 = manual & suggested gear, 3 = auto
/// pit_assist:                         0 = off, 1 = on
/// pit_relase_assist:                  0 = off, 1 = on
/// ers_assist:                         0 = off, 1 = on
/// drs_assist:                         0 = off, 1 = on
/// dynamic_racing_line:                0 = off, 1 = corners only, 2 = full
/// dynamic_racing_line_type:           0 = 2D, 1 = 3D
/// game_mode:                          Game mode id
/// rule_set:                           Rule set id
/// time_of_day:                        Local time of day (minutes since midnight)
/// session_length:                     0 = None, 2 = Very Short, 3 = Short, 4 = Medium
///                                     5 = Medium Long, 6 = Long, 7 = Full
/// speed_units_lead_player:            0 = MPH, 1 = KPH
/// temperature_units_lead_player:      0 = Celsius, 1 = Fahrenheit
/// speed_units_secondary_player:       0 = MPH, 1 = KPH
/// temperature_units_secondary_player: 0 = Celsius, 1 = Fahrenheit
/// num_safety_car_periods:             Number of safety cars called during session
/// num_virtual_safety_car_periods:     Number of virtual safety cars called
/// num_red_flag_periods:               Number of red flags called during session
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
    num_weather_forecast_samples: u8,
    // weather_forecast_samples: [RawWeatherForecast; NUMBER_WEATHER_FORECASTS],
    weather_forecast_samples_1: [RawWeatherForecast; 32], // FIXME: https://stackoverflow.com/a/62665880
    weather_forecast_samples_2: [RawWeatherForecast; 24],
    forecast_accuracy: u8,
    ai_difficulty: u8,
    season_identifier: u32,
    weekend_identifier: u32,
    session_identifier: u32,
    pit_stop_window_ideal_lap: u8,
    pit_stop_window_latest_lap: u8,
    pit_stop_rejoin_position: u8,
    steering_assist: bool,
    braking_assist: u8,
    gearbox_assist: u8,
    pit_assist: bool,
    pit_relase_assist: bool,
    ers_assist: bool,
    drs_assist: bool,
    dynamic_racing_line: u8,
    dynamic_racing_line_type: u8,
    game_mode: u8,
    rule_set: u8,
    time_of_day: u32,
    session_length: u8,
    speed_units_lead_player: u8,
    temperature_units_lead_player: u8,
    speed_units_secondary_player: u8,
    temperature_units_secondary_player: u8,
    num_safety_car_periods: u8,
    num_virtual_safety_car_periods: u8,
    num_red_flag_periods: u8,
}

/// Description of a marshal zone
///
/// ## Specification
/// ```text
/// zone_start: Fraction (0..1) of way through the lap the marshal zone starts
/// zone_flag:  -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow
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

        Ok(Self {
            zone_start: mz.zone_start,
            zone_flag,
        })
    }
}

/// Description of a weather forecast sample
///
/// ## Specification
/// ```text
/// session_type:             0 = unknown, 1 = P1, 2 = P2, 3 = P3, 4 = Short P, 5 = Q1
///                           6 = Q2, 7 = Q3, 8 = Short Q, 9 = OSQ, 10 = R, 11 = R2
///                           12 = Time Trial
/// time_offset:              Time in minutes the forecast is for
/// weather:                  Weather - 0 = clear, 1 = light cloud, 2 = overcast
///                           3 = light rain, 4 = heavy rain, 5 = storm
/// track_temperature:        Track temperature in celsius.
/// track_temperature_change: Track temp. change – 0 = up, 1 = down, 2 = no change
/// air_temperature:          Air temperature in celsius.
/// air_temperature_change:   Air temp. change – 0 = up, 1 = down, 2 = no change
/// rain_percentage:          Rain percentage (0-100)
/// ```
#[derive(Deserialize)]
struct RawWeatherForecast {
    session_type: u8,
    time_offset: u8,
    weather: u8,
    track_temperature: i8,
    track_temperature_change: i8,
    air_temperature: i8,
    air_temperature_change: i8,
    rain_percentage: u8,
}

impl TryFrom<&RawWeatherForecast> for WeatherForecastSample {
    type Error = UnpackError;

    fn try_from(wf: &RawWeatherForecast) -> Result<Self, Self::Error> {
        let session_type = unpack_session_type(wf.session_type)?;
        let weather = unpack_weather(wf.weather)?;
        let track_temperature_change = unpack_temperature_change(wf.track_temperature_change)?;
        let air_temperature_change = unpack_temperature_change(wf.air_temperature_change)?;

        Ok(Self {
            session_type,
            time_offset: wf.time_offset,
            weather,
            track_temperature_change,
            track_temperature: wf.track_temperature,
            air_temperature_change,
            air_temperature: wf.air_temperature,
            rain_percentage: wf.rain_percentage,
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
    let forecast_accuracy = unpack_forecast_accuracy(session_data.forecast_accuracy)?;
    let braking_assist = unpack_braking_assist(session_data.braking_assist)?;
    let gearbox_assist = unpack_gearbox_assist(session_data.gearbox_assist)?;
    let dynamic_racing_line = unpack_dynamic_racing_line(session_data.dynamic_racing_line)?;
    let dynamic_racing_line_type =
        unpack_dynamic_racing_line_type(session_data.dynamic_racing_line_type)?;
    let game_mode = unpack_game_mode(session_data.game_mode)?;
    let rule_set = unpack_rule_set(session_data.rule_set)?;
    let session_length = unpack_session_length(session_data.session_length)?;
    let speed_units_lead_player = unpack_speed_units(session_data.speed_units_lead_player)?;
    let temperature_units_lead_player =
        unpack_temperature_units(session_data.temperature_units_lead_player)?;
    let speed_units_secondary_player =
        unpack_speed_units(session_data.speed_units_secondary_player)?;
    let temperature_units_secondary_player =
        unpack_temperature_units(session_data.temperature_units_secondary_player)?;

    let weather_forecast_samples = session_data
        .weather_forecast_samples_1
        .iter()
        .chain(session_data.weather_forecast_samples_2.iter())
        .take(session_data.num_weather_forecast_samples as usize)
        .map(|wf| wf.try_into())
        .collect::<Result<Vec<WeatherForecastSample>, UnpackError>>()?;

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
        weather_forecast: Some(WeatherForecast {
            number_of_samples: session_data.num_weather_forecast_samples,
            samples: weather_forecast_samples,
            accuracy: forecast_accuracy,
        }),
        ai_difficulty: Some(session_data.ai_difficulty),
        season_identifier: Some(session_data.season_identifier),
        weekend_identifier: Some(session_data.weekend_identifier),
        session_identifier: Some(session_data.session_identifier),
        pit_stop_window_ideal_lap: Some(session_data.pit_stop_window_ideal_lap),
        pit_stop_window_latest_lap: Some(session_data.pit_stop_window_latest_lap),
        pit_stop_rejoin_position: Some(session_data.pit_stop_rejoin_position),
        driving_assists: Some(DrivingAssists {
            steering_assist: session_data.steering_assist,
            braking_assist,
            gearbox_assist,
            pit_assist: session_data.pit_assist,
            pit_relase_assist: session_data.pit_relase_assist,
            ers_assist: session_data.ers_assist,
            drs_assist: session_data.drs_assist,
            dynamic_racing_line,
            dynamic_racing_line_type,
        }),
        game_mode: Some(game_mode),
        rule_set: Some(rule_set),
        time_of_day: Some(session_data.time_of_day),
        session_length: Some(session_length),
        speed_units_lead_player: Some(speed_units_lead_player),
        temperature_units_lead_player: Some(temperature_units_lead_player),
        speed_units_secondary_player: Some(speed_units_secondary_player),
        temperature_units_secondary_player: Some(temperature_units_secondary_player),
        num_safety_car_periods: Some(session_data.num_safety_car_periods),
        num_virtual_safety_car_periods: Some(session_data.num_virtual_safety_car_periods),
        num_red_flag_periods: Some(session_data.num_red_flag_periods),
    })
}
